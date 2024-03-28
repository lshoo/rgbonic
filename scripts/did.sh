#!/usr/bin/env bash

source ./scripts/did.utils.sh

CANISTERS=steward

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister" "actors/$canister"
done

