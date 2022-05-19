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
  --initArgs '{"owner_id": "crossword.'$ACCOUNT_ID'"}'`

## Invoking the methods
```
$ localnear call crossword.$ACCOUNT_ID new_puzzle '{
  "solution_hash": "d1a5cf9ad1adefe0528f7d31866cf901e665745ff172b96892693769ad284010",
  "answers": [
   {
     "num": 1,
     "start": {
       "x": 1,
       "y": 1
     },
     "direction": "Down",
     "length": 5,
     "clue": "NFT market on NEAR that specializes in cards and comics."
   },
   {
     "num": 2,
     "start": {
       "x": 0,
       "y": 2
     },
     "direction": "Across",
     "length": 13,
     "clue": "You can move assets between NEAR and different chains, including Ethereum, by visiting ______.app"
   },
   {
     "num": 3,
     "start": {
       "x": 9,
       "y": 1
     },
     "direction": "Down",
     "length": 8,
     "clue": "NFT market on NEAR with art, physical items, tickets, and more."
   },
   {
     "num": 4,
     "start": {
       "x": 3,
       "y": 8
     },
     "direction": "Across",
     "length": 9,
     "clue": "The smallest denomination of the native token on NEAR."
   },
   {
     "num": 5,
     "start": {
       "x": 5,
       "y": 8
     },
     "direction": "Down",
     "length": 3,
     "clue": "You typically deploy a smart contract with the NEAR ___ tool."
   }
  ]
}' --accountId crossword.$ACCOUNT_ID
```

`$ localnear call crossword.$ACCOUNT_ID submit_solution '{"solution": "paras rainbowbridge mintbase yoctonear cli", "memo": "My memo"}' --accountId $ACCOUNT_ID`

`$ localnear view crossword.$ACCOUNT_ID get_puzzle_status '{"solution_hash": "d1a5cf9ad1adefe0528f7d31866cf901e665745ff172b96892693769ad284010"}' --accountId $ACCOUNT_ID`

`$ localnear view crossword.$ACCOUNT_ID get_solution '{"puzzle_index": 0}' --accountId $ACCOUNT_ID`

`$ localnear view crossword.$ACCOUNT_ID get_unsolved_puzzles --accountId $ACCOUNT_ID`
