# Refactor Plan: OpenAPI Integration

This plan outlines how to integrate Revolut's official OpenAPI specifications while preserving the unique value of this crate's smart client layer.

## Executive Summary

The goal is to reduce maintenance burden by auto-generating data types (structs, enums) from the official OpenAPI specs, while keeping the intelligent client layer that provides:

- Type-state patterns for compile-time correctness
- Automatic OAuth token refresh
- Environment-specific API availability
- Builder patterns and retry middleware

## Current Architecture

```
src/
├── client/              # Generic client infrastructure
│   ├── mod.rs           # Client<E, A> struct, HttpMethod, Body
│   └── builder.rs       # ClientBuilder with type-state
├── business/
│   ├── mod.rs           # Convenience methods on Client
│   ├── client.rs        # BusinessAuthentication, OAuth logic
│   ├── accounts/        # Account types + list/retrieve functions
│   ├── cards/           # Card types + CRUD functions
│   ├── counterparties/  # Counterparty types + functions
│   ├── team_members/    # TeamMember types + functions
│   ├── transactions/    # Transaction types + functions
│   ├── transfers/       # Transfer types + functions
│   ├── webhooks/        # Webhook types (v1, v2)
│   └── simulations/     # Sandbox-only simulation endpoints
├── merchant/
│   ├── mod.rs
│   ├── client.rs        # MerchantAuthentication
│   └── v2024_09_01/     # Versioned merchant types
├── open_banking/
│   ├── mod.rs
│   └── client.rs        # OpenBankingAuthentication
└── errors/              # Error types
```

## Target Architecture

```
src/
├── client/              # KEEP: Smart client infrastructure
├── business/
│   ├── mod.rs           # KEEP: Convenience methods
│   ├── client.rs        # KEEP: OAuth logic, token refresh
│   └── generated/       # NEW: Auto-generated from OpenAPI
│       ├── mod.rs
│       ├── types.rs     # Structs: Account, Card, Transaction, etc.
│       └── enums.rs     # Enums: CardState, TransactionType, etc.
├── merchant/
│   ├── mod.rs           # KEEP
│   ├── client.rs        # KEEP
│   └── generated/       # NEW: From merchant-2024-09-01.yaml (or latest)
├── open_banking/
│   ├── mod.rs           # KEEP
│   ├── client.rs        # KEEP
│   └── generated/       # NEW: From open-banking.yaml
├── crypto_ramp/         # NEW: Not currently implemented
│   ├── mod.rs
│   ├── client.rs
│   └── generated/       # From crypto-ramp-2.0.yaml
└── errors/              # KEEP
```

## What to Keep (Smart Client Layer)

These components provide unique value and cannot be generated:

### 1. Type-State Client Pattern

```rust
// src/client/mod.rs
pub struct Client<E: Environment, T> {
    pub environment: E,
    pub client: reqwest_middleware::ClientWithMiddleware,
    pub authentication: T,
}
```

### 2. Environment Type-State

```rust
// Compile-time enforcement of environment-specific APIs
pub struct SandboxEnvironment<C> { ... }
pub struct ProductionEnvironment<C> { ... }

// Cards only available in production
impl Client<ProductionEnvironment<BusinessClient>, BusinessAuthentication> {
    pub async fn cards(&self) -> ApiResult<...> { ... }
}

// Simulations only available in sandbox
impl Client<SandboxEnvironment<BusinessClient>, BusinessAuthentication> {
    pub async fn transfer_state_update(&self) -> ApiResult<...> { ... }
}
```

### 3. Authentication Builders with Type-State

```rust
// Enforces valid credential combinations at compile time
pub struct BusinessAuthenticationBuilder<A, C, R> {
    client_assertion: A,      // Required
    authorization_code: C,    // Either this...
    refresh_token: R,         // ...or this (mutually exclusive in practice)
}
```

### 4. Automatic Token Refresh

```rust
// src/business/client.rs
impl BusinessAuthentication {
    pub async fn ensure_logged_in(&self, client: &Client<...>) -> ApiResult<String> {
        // Check expiry, refresh if needed, return valid token
    }
}
```

### 5. Retry Middleware

```rust
let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
```

### 6. Error Handling

The structured `Error`, `ClientError`, `BackendError` types with Revolut-specific error parsing.

## What to Generate from OpenAPI

### 1. Request/Response Structs

Currently hand-written in files like `src/business/accounts/v10.rs`:

```rust
// BEFORE (hand-written)
#[derive(Clone, Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub balance: f64,
    pub currency: String,
    pub state: AccountState,
    pub public: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

After generation, these would come from the OpenAPI spec automatically.

### 2. Enums

```rust
// BEFORE (hand-written)
#[derive(Clone, Debug, Deserialize, Serialize, strum::Display)]
#[serde(rename_all = "snake_case")]
pub enum CardState {
    Active,
    Inactive,
    Frozen,
    Terminated,
}
```

### 3. Endpoint Paths

The OpenAPI spec defines all paths. We can generate constants or a registry:

```rust
// Generated
pub mod paths {
    pub const ACCOUNTS_LIST: &str = "/1.0/accounts";
    pub const ACCOUNTS_GET: &str = "/1.0/accounts/{id}";
    pub const CARDS_LIST: &str = "/1.0/cards";
    // ...
}
```

## Implementation Plan

### Phase 1: Setup Code Generation Pipeline

**Objective**: Establish build-time code generation from OpenAPI specs.

#### Step 1.1: Add OpenAPI Specs as Git Submodule

```bash
git submodule add https://github.com/revolut-engineering/revolut-openapi.git openapi-specs
```

Alternatively, fetch specs at build time:

```rust
// build.rs
fn fetch_openapi_specs() {
    // Download from GitHub if not present
}
```

#### Step 1.2: Evaluate Code Generators

| Tool | Pros | Cons |
|------|------|------|
| `openapi-generator` (Java) | Mature, many templates | Requires Java, verbose output |
| `progenitor` (Rust) | Pure Rust, good output | Less flexible customization |
| Custom with `serde_yaml` | Full control | More work |

**Recommendation**: Start with `progenitor` for Rust-native generation, fall back to custom if needed.

#### Step 1.3: Create Build Script

```rust
// build.rs
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=openapi-specs/");

    generate_types("openapi-specs/yaml/business.yaml", "src/business/generated/");
    generate_types("openapi-specs/yaml/merchant-2024-09-01.yaml", "src/merchant/generated/");
    generate_types("openapi-specs/yaml/open-banking.yaml", "src/open_banking/generated/");
}
```

### Phase 2: Generate Types for Business API

**Objective**: Replace hand-written Business API types with generated ones.

#### Step 2.1: Generate Types

Run generator against `business.yaml`, producing:

```
src/business/generated/
├── mod.rs
├── accounts.rs      # Account, AccountState, etc.
├── cards.rs         # Card, CardState, SpendingLimit, etc.
├── counterparties.rs
├── transactions.rs
├── transfers.rs
├── webhooks.rs
└── common.rs        # Shared types (Money, Address, etc.)
```

#### Step 2.2: Create Compatibility Layer

To avoid breaking changes, re-export generated types from existing module paths:

```rust
// src/business/accounts/mod.rs
pub mod v10 {
    // Re-export generated types
    pub use crate::business::generated::accounts::*;

    // Keep hand-written request functions
    pub async fn list<E: Environment>(
        client: &Client<E, BusinessAuthentication>,
    ) -> ApiResult<Vec<Account>> {
        // ...
    }
}
```

#### Step 2.3: Validate Generated Types

Compare generated types against existing hand-written types:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_serialization_compatibility() {
        let json = r#"{"id": "...", ...}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        // Verify all fields parse correctly
    }
}
```

### Phase 3: Generate Types for Merchant API

**Objective**: Generate Merchant API types, supporting multiple versions.

#### Step 3.1: Version Strategy

The OpenAPI repo has multiple merchant versions:
- `merchant-1.0.yaml`
- `merchant-2023-09-01.yaml`
- `merchant-2024-05-01.yaml`
- `merchant-2024-09-01.yaml`
- `merchant-2025-10-16.yaml`
- `merchant-2025-12-04.yaml`

Strategy: Generate for supported versions, keep the `pub use latest` pattern:

```rust
// src/merchant/mod.rs
pub mod v2024_09_01 {
    pub use crate::merchant::generated::v2024_09_01::*;
}

pub mod v2025_12_04 {
    pub use crate::merchant::generated::v2025_12_04::*;
}

pub use v2025_12_04 as latest;
```

#### Step 3.2: Generate Each Version

```rust
// build.rs
for version in ["2024-09-01", "2025-12-04"] {
    generate_types(
        &format!("openapi-specs/yaml/merchant-{}.yaml", version),
        &format!("src/merchant/generated/v{}/", version.replace("-", "_")),
    );
}
```

### Phase 4: Add Crypto Ramp API (New)

**Objective**: Add support for the Crypto Ramp API, which is in the OpenAPI specs but not in this crate.

#### Step 4.1: Create Module Structure

```
src/crypto_ramp/
├── mod.rs           # Convenience methods
├── client.rs        # CryptoRampAuthentication (determine auth method)
└── generated/
    └── v2/          # From crypto-ramp-2.0.yaml
```

#### Step 4.2: Implement Client

Analyze `crypto-ramp-2.0.yaml` to determine authentication requirements, then implement appropriate type-state patterns.

### Phase 5: Generate Types for Open Banking API

**Objective**: Generate Open Banking types.

Similar to Business API - generate types, create compatibility layer, validate.

### Phase 6: Refine and Optimize

#### Step 6.1: Custom Derive Macros

If generated code lacks certain derives, create post-processing or custom templates:

```rust
// Ensure all enums have Display
#[derive(Clone, Debug, Deserialize, Serialize, strum::Display)]
pub enum CardState { ... }
```

#### Step 6.2: Documentation

Generated types should include doc comments from OpenAPI descriptions:

```rust
/// A business account.
///
/// Accounts hold balances in specific currencies and can be used
/// for transfers and payments.
#[derive(Clone, Debug, Deserialize)]
pub struct Account {
    /// Unique identifier for the account
    pub id: String,
    // ...
}
```

#### Step 6.3: Deprecation Handling

If OpenAPI marks fields as deprecated, propagate to generated code:

```rust
#[deprecated(note = "Use `new_field` instead")]
pub old_field: Option<String>,
```

## Migration Strategy

### For Existing Users

1. **Minor version bump** for Phase 2-5 (types come from generation but API is same)
2. **No breaking changes** to public API
3. Re-exports ensure `use revolut::business::accounts::v10::Account` still works

### Deprecation Path

If we want to eventually expose generated modules directly:

```rust
// Phase 1: Re-export (no breaking change)
pub mod accounts {
    pub mod v10 {
        pub use crate::business::generated::accounts::*;
    }
}

// Phase 2: Deprecate old path (major version)
#[deprecated(since = "2.0.0", note = "Use `generated::accounts` directly")]
pub mod accounts { ... }
```

## Build Configuration

### Cargo.toml Changes

```toml
[build-dependencies]
progenitor = "0.x"  # Or chosen generator
serde_yaml = "0.9"

[features]
default = []
# Regenerate types (for development)
regenerate-types = []
```

### CI/CD Considerations

1. **Check generated code into git** - Ensures reproducible builds without generator at build time
2. **CI job to verify generation** - Run generator, compare output, fail if different
3. **Dependabot/Renovate** - Watch `revolut-openapi` for updates

```yaml
# .github/workflows/check-openapi.yml
name: Check OpenAPI Sync
on:
  schedule:
    - cron: '0 0 * * 1'  # Weekly
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: true
      - run: cargo build --features regenerate-types
      - run: git diff --exit-code src/*/generated/
```

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Generated code doesn't compile | Medium | High | Comprehensive test suite, manual review |
| Breaking changes in OpenAPI spec | Low | High | Pin to specific commit/tag, review before updating |
| Generator produces non-idiomatic code | Medium | Medium | Custom templates, post-processing |
| Missing fields in OpenAPI spec | Low | Medium | Report upstream, add manual overrides |

## Success Criteria

1. All existing tests pass with generated types
2. No public API breaking changes
3. Build time increase < 10 seconds
4. Generated code is readable and well-documented
5. Crypto Ramp API fully supported

## Timeline Considerations

This plan is broken into phases that can be implemented incrementally. Each phase is independently valuable:

- **Phase 1** alone provides the infrastructure
- **Phase 2** alone reduces Business API maintenance
- **Phase 4** alone adds new functionality (Crypto Ramp)

Phases can be reordered based on priority.

## Open Questions

1. **Which generator to use?** - Needs evaluation of `progenitor` vs `openapi-generator` vs custom
2. **Submodule vs fetch?** - Git submodule is simpler but adds repo size; fetching is cleaner but needs network
3. **Which merchant versions to support?** - All of them? Just latest + one previous?
4. **Crypto Ramp authentication?** - Need to analyze the OpenAPI spec to determine auth method

## Appendix: Files to Keep vs Generate

### Keep (Smart Client Layer)

| File | Reason |
|------|--------|
| `src/client/*` | Type-state client infrastructure |
| `src/business/client.rs` | OAuth token refresh logic |
| `src/business/mod.rs` | Environment-specific method impls |
| `src/merchant/client.rs` | Secret key authentication |
| `src/open_banking/client.rs` | Authentication logic |
| `src/errors/*` | Custom error handling |

### Generate (Data Types)

| Current File | Generated From |
|--------------|----------------|
| `src/business/accounts/v10.rs` | `business.yaml` → `#/components/schemas/Account*` |
| `src/business/cards/v10.rs` | `business.yaml` → `#/components/schemas/Card*` |
| `src/business/counterparties/v10.rs` | `business.yaml` → `#/components/schemas/Counterparty*` |
| `src/business/transactions/v10.rs` | `business.yaml` → `#/components/schemas/Transaction*` |
| `src/business/transfers/v10.rs` | `business.yaml` → `#/components/schemas/Transfer*` |
| `src/business/webhooks/v1.rs` | `business.yaml` → `#/components/schemas/Webhook*` |
| `src/business/webhooks/v2.rs` | `business.yaml` → `#/components/schemas/Webhook*` (v2) |
| `src/merchant/v2024_09_01/*` | `merchant-2024-09-01.yaml` |

### New (From OpenAPI, Not Currently Implemented)

| To Generate | From |
|-------------|------|
| `src/crypto_ramp/generated/*` | `crypto-ramp-2.0.yaml` |
