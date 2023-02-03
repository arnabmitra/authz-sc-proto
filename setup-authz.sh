#!/bin/bash

export PROVENANCE_DIR="$HOME/provenance"
export BIN="$PROVENANCE_DIR/build/provenanced"
export RUN_HOME="$PROVENANCE_DIR/build/run/provenanced"
export GAS_FLAGS="--gas auto --gas-prices 1905nhash --gas-adjustment 1.5"
export CHAIN="$BIN -t --home $RUN_HOME"
export VALIDATOR1=$($CHAIN keys show validator -a)
export CONTRACT_ADDRESS="tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8"
#printf $VALIDATOR1

${CHAIN} tx authz grant $CONTRACT_ADDRESS send --spend-limit=100000000000nhash  \
    --from $VALIDATOR1 --fees 100000000000nhash --chain-id testing --keyring-backend test --yes -o json  | jq
