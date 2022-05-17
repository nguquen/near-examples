## Test the code

`$ cargo test -- --nocapture`

## Compile the code

`$ cargo build --target wasm32-unknown-unknown --release`

## Deploying the smart contract

### Create sub account

`$ local_near create-account status-message.$ACCOUNT_ID --masterAccount $ACCOUNT_ID --initialBalance 20`

### Deploy the smart contract

`$ local_near deploy status-message.$ACCOUNT_ID --wasmFile ./target/wasm32-unknown-unknown/release/status_message.wasm`

## Invoking the methods

`$ local_near call status-message.$ACCOUNT_ID set_status '{"message": "hello"}' --accountId $ACCOUNT_ID`

`$ local_near view status-message.$ACCOUNT_ID get_status '{"account_id": "'$ACCOUNT_ID'"}' --accountId $ACCOUNT_ID`
