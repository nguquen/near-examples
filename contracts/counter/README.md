## Test the code

`$ cargo test -- --nocapture`

## Compile the code

`$ cargo build --target wasm32-unknown-unknown --release`

## Deploying the smart contract

### Create sub account

`$ localnear create-account counter.$ACCOUNT_ID --masterAccount $ACCOUNT_ID --initialBalance 20`

### Deploy the smart contract

`$ localnear deploy counter.$ACCOUNT_ID --wasmFile ./target/wasm32-unknown-unknown/release/counter.wasm`

## Invoking the methods

`$ localnear call counter.$ACCOUNT_ID increment --accountId $ACCOUNT_ID`

`$ localnear view counter.$ACCOUNT_ID get_num --accountId $ACCOUNT_ID`

`$ localnear call counter.$ACCOUNT_ID decrement --accountId $ACCOUNT_ID`
