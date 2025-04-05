#!/bin/bash

set -e

# --- Optimize the factory contract
echo "Optimizing factory contract..."
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0
echo "Factory contract optimized."

# --- Upload factory contract on-chain
WALLET="xion18x6a3f8mm6472r7qkf3svk56yducnzsd8f9trl" # Replace with your wallet address
CHAIN_ID="xion-testnet-2"
RPC_NODE="https://rpc.xion-testnet-2.burnt.com:443"

echo "Uploading factory contract..."
FACTORY_RES=$(xiond tx wasm store ./artifacts/factory.wasm \
    --chain-id $CHAIN_ID \
    --gas-adjustment 1.3 \
    --gas-prices 0.1uxion \
    --gas auto \
    -y --output json \
    --node $RPC_NODE \
    --from $WALLET)
echo "Factory Upload Response: $FACTORY_RES"

# --- Retrieve FACTORY CODE ID
FACTORY_TXHASH=$(echo "$FACTORY_RES" | jq -r '.txhash')
echo "Factory Upload TX Hash: $FACTORY_TXHASH"
FACTORY_CODE_ID=$(xiond query tx $FACTORY_TXHASH \
    --node $RPC_NODE \
    --output json | jq -r '.logs[0].events[] | select(.type == "wasm") | .attributes[] | select(.key == "code_id") | .value')
echo "FACTORY_CODE_ID: $FACTORY_CODE_ID"

# --- Optimize the game contract
echo "Optimizing game contract..."
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache_game",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.0 -o ./artifacts/game.wasm
echo "Game contract optimized."

# --- Upload game contract on-chain
echo "Uploading game contract..."
GAME_RES=$(xiond tx wasm store ./artifacts/game.wasm \
    --chain-id $CHAIN_ID \
    --gas-adjustment 1.3 \
    --gas-prices 0.1uxion \
    --gas auto \
    -y --output json \
    --node $RPC_NODE \
    --from $WALLET)
echo "Game Upload Response: $GAME_RES"

# --- Retrieve GAME CODE ID
GAME_TXHASH=$(echo "$GAME_RES" | jq -r '.txhash')
echo "Game Upload TX Hash: $GAME_TXHASH"
GAME_CODE_ID=$(xiond query tx $GAME_TXHASH \
    --node $RPC_NODE \
    --output json | jq -r '.logs[0].events[] | select(.type == "wasm") | .attributes[] | select(.key == "code_id") | .value')
echo "GAME_CODE_ID: $GAME_CODE_ID"

# --- Instantiate factory contract
FACTORY_INSTANTIATE_MSG='{
  "admin": "'"$WALLET"'",
  "game_code_id": '"$GAME_CODE_ID"',
  "admin_fee_percentage": 5,
  "player_win_percentage": 80,
  "player_draw_percentage": 15
}'
echo "Instantiating factory contract with MSG: $FACTORY_INSTANTIATE_MSG"
FACTORY_INITIALIZE_TXHASH=$(xiond tx wasm instantiate "$FACTORY_CODE_ID" "$FACTORY_INSTANTIATE_MSG" \
  --from $WALLET \
  --label "game-factory" \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y --no-admin \
  --chain-id $CHAIN_ID \
  --node $RPC_NODE \
  --output json | jq -r '.logs[0].events[] | select(.type == "wasm") | .attributes[] | select(.key == "_contract_address") | .value')
FACTORY_CONTRACT_ADDRESS=CONTRACT=$(xiond query tx $FACTORY_INITIALIZE_TXHASH \
  --node https://rpc.xion-testnet-2.burnt.com:443 \
  --output json | jq -r '.events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
echo "FACTORY_CONTRACT_ADDRESS: $FACTORY_CONTRACT_ADDRESS"

echo "Factory contract deployed and instantiated successfully!"
echo "Factory Contract Address: $FACTORY_CONTRACT_ADDRESS"
echo "Game Code ID: $GAME_CODE_ID"