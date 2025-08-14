default: fmt lint build test

fmt:
  find . -name "*.nix" | xargs alejandra
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

create-business-webhook: (run-example "create_business_webhook")

list-accounts: (run-example "list_accounts")

list-business-webhooks: (run-example "list_business_webhooks")

# --- Merchant API

cancel-order order_id: (run-example "cancel_order" "--order-id" order_id)

capture-order order_id amount: (run-example "capture_order" "--order-id" order_id "--amount" amount)

create-merchant-webhook: (run-example "create_merchant_webhook")

create-order amount currency: (run-example "create_order" "--amount" amount "--currency" currency)

list-merchant-webhooks: (run-example "list_merchant_webhooks")

list-orders: (run-example "list_orders")

list-orders-tidy:
    just run-example "list_orders" | jq -r '.[] | .id + ": " + .state'
