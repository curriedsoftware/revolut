# OpenAPI spec overrides

The `openapi-specs/` submodule tracks Revolut's published OpenAPI specs
(`revolut-engineering/revolut-openapi`). A few fields are documented and work
against the live API but are **missing from the upstream schema**, so codegen
can't emit them. We keep minimal local patches here and apply them to the
submodule before running `just generate`.

Each `*.patch` is a `git diff` taken inside the `openapi-specs/` submodule. To
(re)apply after a fresh checkout or a submodule bump:

```bash
cd openapi-specs
git apply ../openapi-specs-overrides/*.patch
cd ..
just generate
```

If `git apply` fails because line numbers moved, re-add the field by hand
(the patches are tiny) and refresh the patch with
`git -C openapi-specs diff -- <spec> > openapi-specs-overrides/<name>.patch`.

## Patches

### `merchant-payment_method_id.patch`

Adds `payment_method_id` to the `Subscription-Creation` schema in
`merchant-2026-04-20.yaml` (the version the `generate` target uses).

**Why:** the `createSubscription` endpoint accepts `payment_method_id` to attach a
payment method already saved for the customer, activating the subscription
off-session (no Hosted Payment Page). Revolut's own request examples
(`Req-Subscription-With-Trial`, `Req-Subscription-Skip-Trial`) include it, and it
is verified working against the sandbox (a create with a valid saved-card id
returns `setup_order_id: null` and the subscription goes `active`; the field is
honoured, not ignored). But the upstream `Subscription-Creation` *schema* omits
it — in every spec version (`1.0` … `2026-04-20`) — so it is an upstream
spec/schema inconsistency, not a versioning issue. Remove this patch once
upstream declares the field.
