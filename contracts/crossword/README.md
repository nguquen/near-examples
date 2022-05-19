## Test the code

`$ cargo unit-test -- --nocapture`

## Compile the code

`$ cargo wasm`

## Deploying the smart contract

### Create sub account

`$ localnear create-account crossword.$ACCOUNT_ID --masterAccount $ACCOUNT_ID --initialBalance 20`

### Deploy the smart contract

`$ localnear deploy crossword.$ACCOUNT_ID --wasmFile ./target/wasm32-unknown-unknown/release/crossword.wasm \
  --initFunction 'new' \
  --initArgs '{"solution": "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f"}'`

## Invoking the methods

`$ localnear call crossword.$ACCOUNT_ID guess_solution '{"solution": "near nomicon ref finance"}' --accountId $ACCOUNT_ID`

`$ localnear view crossword.$ACCOUNT_ID get_solution --accountId $ACCOUNT_ID`
