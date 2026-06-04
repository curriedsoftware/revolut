default: fmt lint build test

generate:
  cd /tmp && cargo run --manifest-path {{justfile_directory()}}/codegen/Cargo.toml -- --spec {{justfile_directory()}}/openapi-specs/yaml/business.yaml --output {{justfile_directory()}}/src/business/generated/ --api business
  cd /tmp && cargo run --manifest-path {{justfile_directory()}}/codegen/Cargo.toml -- --spec {{justfile_directory()}}/openapi-specs/yaml/merchant-2026-04-20.yaml --output {{justfile_directory()}}/src/merchant/generated/ --api merchant
  cd /tmp && cargo run --manifest-path {{justfile_directory()}}/codegen/Cargo.toml -- --spec {{justfile_directory()}}/openapi-specs/yaml/open-banking.yaml --output {{justfile_directory()}}/src/open_banking/generated/ --api open_banking
  cd /tmp && cargo run --manifest-path {{justfile_directory()}}/codegen/Cargo.toml -- --spec {{justfile_directory()}}/openapi-specs/yaml/crypto-ramp-2.0.yaml --output {{justfile_directory()}}/src/crypto_ramp/generated/ --api crypto_ramp
  cargo fmt

fmt:
  find . -name "*.nix" -not -path "./vendor/*" | xargs alejandra
  cargo fmt

lint:
  cargo clippy

check-licenses:
  cargo deny check

audit:
  cargo audit -f Cargo.nix.lock --json | jq -e '. as $expression | $expression, ($expression | .vulnerabilities.found | not)'

build:
  cargo build

test:
  cargo test

update-cargo-lock:
  cargo generate-lockfile
  cp Cargo.lock Cargo.nix.lock

run-example example *args='':
  cargo run --example {{ example }} -- {{ args }}

retrieve-access-token: (run-example "retrieve_access_token")

# --- Business API

bank-details account_id: (run-example "bank_details" "--account-id" account_id)

create-business-webhook url *args='': (run-example "create_business_webhook" "--url" url args)

delete-business-webhook webhook_id: (run-example "delete_business_webhook" "--webhook-id" webhook_id)

list-accounts: (run-example "list_accounts")

list-business-webhooks: (run-example "list_business_webhooks")

# --- Merchant API

cancel-order order_id: (run-example "cancel_order" "--order-id" order_id)

capture-order order_id amount: (run-example "capture_order" "--order-id" order_id "--amount" amount)

create-merchant-webhook url *args='': (run-example "create_merchant_webhook" "--url" url args)

create-order amount currency: (run-example "create_order" "--amount" amount "--currency" currency)

delete-merchant-webhook webhook_id: (run-example "delete_merchant_webhook" "--webhook-id" webhook_id)

get-order order_id: (run-example "get_order" "--order-id" order_id)

list-merchant-webhooks: (run-example "list_merchant_webhooks")

list-orders *args='': (run-example "list_orders" args)

list-orders-tidy:
    just run-example "list_orders" | jq -r '.[] | .id + ": " + .state'
