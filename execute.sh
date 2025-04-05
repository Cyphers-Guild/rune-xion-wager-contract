#!/bin/bash

set -e

WALLET="your-wallet-address-here"
FACTORY_CONTRACT_ADDRESS="xion182xdvwf4e98yty99w6y97evtlnawarhzn7n56hlnqy4qcgzqu5rq27gsve"
CHAIN_ID="xion-testnet-2"
RPC_NODE="https://rpc.xion-testnet-2.burnt.com:443"

EXECUTE_MSG='{
  "create_game": {
    "white_player": {
      "name": "fredd",
      "address": "xion19rmwtwvh9wwlkz44n80jylwe283j8e3w33l7qd"
    },
    "black_player": {
      "name": "dann",
      "address": "xion1hs4l5ss3ku469ra3q9zmunxsa4l0srt5nyysvv"
    },
    "wager": {
      "denom": "uxion",
      "amount": "1000000"
    }
  }
}'


echo "Sending execute message to factory to create a game..."
EXECUTE_RES=$(xiond tx wasm execute "$FACTORY_CONTRACT_ADDRESS" "$EXECUTE_MSG" \
  --from $WALLET \
  --gas-prices 0.025uxion \
  --gas auto \
  --gas-adjustment 1.3 \
  -y \
  --chain-id $CHAIN_ID \
  --node $RPC_NODE)
echo "Execute Response: $EXECUTE_RES"