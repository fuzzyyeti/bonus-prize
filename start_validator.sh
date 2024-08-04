#!/bin/bash

## Rewrite the winner to key in KEY_PATH
pushd sdk || exit
ts-node ./accountSetup/rewriteWinner.ts
popd || exit

# Start the validator
solana-test-validator --reset --account EwZgGG2Q2eE5ygTGDJZ9c161k87d8qbQWMqPDknyeW6U ./sdk/accountSetup/newDrawResult.json &

sleep 10

# Deploy the program
solana program deploy ./programs/bonus_prize/target/deploy/bonus_prize.so

