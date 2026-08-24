# Verification-tier policy

This document defines the bounded and authorized write path for verification
tiers. A tier is a security-sensitive claim: it affects verification queries,
directory filtering, and downstream decisions. It is therefore not treated as
an ordinary profile field.

## Policy

The contract uses a closed range from `0` through `10`.

| Value | Meaning | Write rule |
| ---: | --- | --- |
| 0 | Not verified | The lower bound; downgrade is idempotent here. |
| 1–9 | Progressive verification | The configured tier administrator may transition by one or set explicitly. |
| 10 | Highest supported verification | Bump is rejected instead of wrapping. |

The maximum is a protocol constant, not a caller-supplied limit. Changing it is
a protocol migration and must be reviewed as an authorization-sensitive change.

## Authorization model

`initialize_tier_admin(admin)` stores one administrator address in instance
storage and requires that address to authorize initialization. Initialization is
write-once. The explicit methods `set_tier_authorized`,
`bump_tier_authorized`, `downgrade_tier_authorized`, and
`set_profile_authorized` all require both:

1. an authorization from the supplied address; and
2. an exact match with the stored tier administrator.

The contract does not infer an administrator from the first tier write. This
prevents a race between deployment and policy setup. A deployment script should
initialize the policy before registering or importing any verified records.

For compatibility with the original API, legacy mutators remain callable on an
uninitialized instance. Once `initialize_tier_admin` has been called, those
mutators enforce the stored administrator and the same maximum. This provides a
safe migration path for existing callers while making initialized deployments
strict. New integrations should use the explicit methods so the authority is
visible in the call shape.

## Transition behavior

Every checked write validates the requested tier before storage mutation. A
successful explicit write returns a `TierTransition` containing:

- business ID;
- previous tier;
- next tier; and
- a bounded reason symbol (`set`, `bump`, or `downgrade`).

The contract also emits `tier_changed` with that transition. Indexers can use
the transition to audit policy changes without reconstructing state from every
profile read.

`bump` rejects a current value of `10`; it never performs wrapping arithmetic.
`downgrade` uses checked subtraction and maps a zero predecessor to zero. An
explicit set rejects values above `10` before it touches the tier map.

## Compound profile updates

`set_profile_authorized` validates the tier and the administrator before writing
category, tier, or active state. It writes all three maps only after validation,
then returns `get_profile` from the completed state. An unauthorized or invalid
call therefore cannot update category or active state as a side effect of a
failed tier request.

Soroban invocation rollback additionally means a trapped call does not expose a
partial persistent update. The tests assert the successful return value and
the full post-call profile rather than treating the individual maps as an API.

## Failure behavior

The contract traps on:

- a second policy initialization;
- an uninitialized explicit authorized operation;
- a mismatched administrator address;
- a missing administrator authorization;
- a tier greater than `MAX_VERIFICATION_TIER`; or
- a bump from the maximum tier.

Trapping is intentional: callers must not receive a success result for a
transition that did not happen. Read methods remain total and return zero,
default category, or the default active state for unknown business IDs as they
did before this policy.

## Compatibility and migration

The storage keys for existing business, category, tier, active, and signal data
are unchanged. The new `tieradmin` instance key is additive, and the transition
event is also additive. Existing read clients can continue to call
`get_verification_tier`, `meets_tier`, `get_profile`, and the aggregate queries.

Migration steps:

1. Deploy the new contract version or upgrade artifact.
2. Confirm the intended governance address and have it call
   `initialize_tier_admin`.
3. Verify `get_tier_admin` returns the expected address.
4. Run a lower-bound, normal, and upper-bound write using the governance key.
5. Confirm indexers consume `tier_changed` and retain the transition payload.
6. Disable legacy mutator use in application clients and adopt explicit methods.

Do not initialize with an operational user key when the deployment is governed
by a multisig. The stored address can be a contract or account that supports
Soroban authorization, subject to the deployment's custody policy.

## Rollback and recovery

If the release must be rolled back before initialization, existing storage is
compatible with the previous code. If the policy key has been initialized, a
rollback must preserve the key or the deployment could accidentally reopen
legacy writes. Prefer rolling forward with a tested recovery method rather than
removing the policy key.

If the administrator is lost, the contract has no implicit recovery authority.
Use the deployment's governed upgrade or account-recovery process. A future
admin-rotation feature must be a separate reviewed change with a two-step handoff
and an explicit event; it must not silently overwrite `tieradmin`.

## Test matrix

The unit suite covers:

- initialization and write-once behavior;
- exact lower and upper bounds;
- set overflow and bump overflow;
- repeated bumps and repeated zero downgrades;
- wrong administrator for set, bump, downgrade, and compound profile;
- initialized legacy-call enforcement;
- complete compound profile observation; and
- default read behavior for unknown businesses.

The maximum-bound tests are especially important because a `u32` increment can
otherwise wrap in release builds if it is not checked. The authorization tests
also cover the mismatch after `require_auth`, ensuring that a valid signature
from the wrong address is not accepted as the role.

## Review checklist

- [ ] The maximum remains documented and tested.
- [ ] Policy initialization happens before verified data writes.
- [ ] Explicit privileged methods require the configured address.
- [ ] Legacy calls are used only during migration and only before initialization.
- [ ] Compound writes validate before mutation.
- [ ] Events include the previous and next tier.
- [ ] No code path uses unchecked increment or decrement.
- [ ] Rollback preserves the policy key once initialized.
- [ ] CI runs the full test suite without disabled checks.
