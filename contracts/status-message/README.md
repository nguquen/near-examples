## Test the code

`$ cargo test -- --nocapture`

## Compile the code

`$ RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

## Deploying the smart contract

### Create sub account

`$ localnear create-account status-message.$ACCOUNT_ID --masterAccount $ACCOUNT_ID --initialBalance 20`

### Deploy the smart contract

`$ localnear deploy status-message.$ACCOUNT_ID --wasmFile ./target/wasm32-unknown-unknown/release/status_message.wasm`

## Invoking the methods

`$ localnear call status-message.$ACCOUNT_ID set_status '{"message": "hello"}' --accountId $ACCOUNT_ID`

`$ localnear view status-message.$ACCOUNT_ID get_status '{"account_id": "'$ACCOUNT_ID'"}' --accountId $ACCOUNT_ID`
