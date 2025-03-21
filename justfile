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

refresh-access-token: (run-example "refresh_access_token")

list-accounts: (run-example "list_accounts")

bank-details account_id: (run-example "bank_details" "--account-id" account_id)
