# Signal replay protection

Signal recording changes trust statistics, so a retried submission must not
silently become a second observation. The authorized signal API provides a
caller-bound identity and an atomic replay barrier while preserving the
original `record_signal` method for existing uninitialized integrations.

## Identity contract

The replay identity is the tuple:

```text
(business_id, signal_type, value, context, submitter, nonce)
```

Every member is part of the key. A submission for another business is not the
same observation. A changed signal type, amount, context, submitter, or nonce is
also a distinct identity, subject to the submitter's monotonic nonce rule.

The submitter is a Soroban `Address` and must authorize the invocation. The
contract does not accept a caller string supplied only as metadata. This binds
the replay record to the address that signed the operation and allows indexers
to attribute the observation consistently.

`context` is an application-defined bounded string. It should contain a stable
business reference such as an invoice, settlement, or evidence identifier. It
must not contain secrets. The contract limits it to 256 characters to keep the
identity and storage cost bounded.

## Submission result

`record_signal_authorized` returns `SignalSubmission`:

| Field | Meaning |
| --- | --- |
| `accepted` | A new observation was appended. |
| `duplicate` | The exact identity was already accepted. |
| `signal_count` | Current count for the target business after the decision. |

An exact retry returns `{ accepted: false, duplicate: true }` and leaves the
signal vector, nonce map, and replay map unchanged. This deterministic response
lets a client safely acknowledge a network timeout after checking the chain.
It does not require the client to guess whether the first submission landed.

## Nonce policy

Nonce values are scoped to the authorized submitter. The first accepted nonce
may be any `u64` value. Each later new identity must use a strictly greater
nonce. A repeated exact identity is checked before the monotonic rule, so a
retry returns the duplicate result instead of being misreported as stale.

This policy catches delayed or reordered submissions. It also means callers
should allocate nonces from one durable per-address sequence. If an application
needs independent concurrent sequences, it should include a sequence family in
the context and coordinate the submitter's outer nonce allocator.

The nonce is not consumed by validation failures. A caller can correct an
oversized context or other client-side error and reuse the intended nonce with
a valid identity. Once a new observation is accepted, however, the nonce moves
forward in the same transaction as the observation.

## Atomicity

The accepted path performs these logical steps inside one contract invocation:

1. require authorization from the submitter;
2. validate bounded input;
3. check the exact replay identity;
4. check the submitter's latest nonce;
5. append the signal record;
6. mark the identity as consumed;
7. update the latest nonce; and
8. persist the three collections and emit the event.

Soroban traps roll back the invocation. A failure after step 5 therefore cannot
leave a nonce or replay marker without its signal, and a failure before the
write cannot burn a retry token. The code keeps these writes in the same entry
point and does not expose a separate “reserve nonce” operation.

## Duplicate and cross-scope behavior

The replay map distinguishes the following cases:

- same full tuple: deterministic duplicate, no new count;
- same nonce with another business: stale for that submitter;
- same business with another type or value: a new identity if its nonce is new;
- same tuple from another submitter: a separate identity;
- same tuple with another context: a separate identity; and
- same tuple with a larger nonce: a separate deliberate observation.

The business and context fields are included because replaying a valid signal
against another business or settlement must not be treated as harmless client
duplication. The submitter field is included because two authorized data sources
may independently report the same raw value.

## Compatibility

The original `record_signal(env, business_id, signal_type, value)` entry point
is unchanged. It remains useful for historical fixtures and existing callers,
but it has no caller-bound replay identity. New production integrations should
use `record_signal_authorized` and store the returned result. A future breaking
migration can remove the legacy entry point after all clients have moved.

The replay and nonce maps use additive persistent storage keys (`replays` and
`nonces`). Existing signal records remain readable by all score and reporting
queries. The new event is additive and contains the business identifier; it does
not expose the context or submitter as an unbounded event payload.

## Failure behavior

The authorized method traps for:

- missing submitter authorization;
- context longer than 256 characters;
- a new submission with a nonce not greater than the submitter's latest nonce;
- storage or host errors while writing the observation; or
- malformed contract values rejected by Soroban deserialization.

An exact replay is not a failure and must not trap. Its explicit duplicate bit
is part of the API contract. Applications should treat a duplicate as success
for retry reconciliation, but should not increment local counters a second time.

## Client retry algorithm

1. Allocate and persist a nonce before submitting.
2. Build the full identity, including stable context.
3. Sign and submit the authorized call.
4. If the response is successful, retain the returned count and transaction ID.
5. If the network response is ambiguous, retry the exact same tuple.
6. If the contract returns duplicate, reconcile using the returned count.
7. If it returns stale, inspect the latest accepted nonce before creating a new
   deliberate observation.

Clients must not change the nonce, amount, or context when retrying an
ambiguous request. Doing so creates a different identity and may intentionally
record another observation.

## Monitoring and operations

Operators should monitor accepted, duplicate, and stale outcomes separately.
Duplicate rates may indicate normal network retries, while a spike in stale
submissions can indicate queue reordering or a broken nonce allocator. Counts
should be compared with provider and indexer records using the context value.

The replay map is correctness state, not a cache. It must be retained for at
least as long as signal records can be replayed. Any archival process must copy
the identity, acceptance transaction, submitter, and nonce before removing old
data. Deleting replay keys while retaining signals would reopen the inflation
path.

## Migration and rollback

No migration is required for existing signal vectors. Before enabling the new
API, application owners should select a nonce namespace for each submitter and
document how retries are recovered. Existing off-chain signal imports should
be assigned stable context values rather than using a random value per retry.

If the release is rolled back, the additive maps should be preserved. Removing
them and later re-enabling the feature would allow an old accepted identity to
be submitted again. A rollback that cannot preserve the maps must disable the
authorized endpoint until a replay-state migration is completed.

## Test matrix

The test suite covers:

- first accepted submission and returned state;
- exact duplicate retries;
- business, type, value, context, submitter, and nonce identity changes;
- stale nonce rejection;
- bounded context validation;
- validation failure followed by nonce reuse;
- direct replay-map queries;
- repeated unique identities; and
- repeated duplicate attempts with unchanged signal counts.

The repeated identity test is intentionally larger than the minimum happy path:
it demonstrates that uniqueness is maintained across a sequence rather than
only for one adjacent retry.

## Review checklist

- [ ] Production callers use the authorized entry point.
- [ ] Context is stable, bounded, and free of secrets.
- [ ] Retries resend the exact same tuple.
- [ ] Nonces are durable and scoped to the submitter.
- [ ] Replay state is retained with signal history.
- [ ] Duplicate outcomes do not increment local or on-chain counts.
- [ ] Stale outcomes are investigated rather than silently retried.
- [ ] Failed writes do not consume a nonce.
- [ ] Full CI runs without disabled checks or generated noise.
