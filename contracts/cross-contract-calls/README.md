## Test the code

`$ cargo test -- --nocapture`

## Compile the code

`$ RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

## Deploying the smart contract

### Create sub account

`$ localnear create-account cross-contract.$ACCOUNT_ID --masterAccount $ACCOUNT_ID --initialBalance 20`

### Deploy the smart contract

`$ localnear deploy cross-contract.$ACCOUNT_ID --wasmFile ./target/wasm32-unknown-unknown/release/cross_contract_calls.wasm`

## Invoking the methods

`$ localnear call cross-contract.$ACCOUNT_ID check_counter '{"ext_contract_id": "counter.'$ACCOUNT_ID'"}' --accountId $ACCOUNT_ID`
