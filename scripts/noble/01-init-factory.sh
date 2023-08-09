# starsd config node $NODE
# starsd config chain-id $CHAIN_ID
# starsd config output json

KEY=$(starsd keys show $ADMIN | jq -r .name)
FACTORY_CODE_ID=2792
MINTER_CODE_ID=2783
DEV_ADDRESS="stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd";

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_sg721_code_ids": [2784],
    "frozen": false,
    "creation_fee": {"amount": "1000000000", "denom": "ustars"},
    "min_mint_price": {"amount": "0", "denom": "factory/stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd/ufrnz"},
    "mint_fee_bps": 10000,
    "max_trading_offset_secs": 0,
    "extension": {
        "max_per_address_limit": 50,
        "airdrop_mint_price": { "denom": "factory/stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd/ufrnz", "amount": "0" },
        "airdrop_mint_fee_bps": 10000,
        "dev_fee_address": "$DEV_ADDRESS"
    }
  }
}

EOF
)
echo $MSG


starsd tx wasm instantiate $FACTORY_CODE_ID "$MSG" --label "NobleFactory" \
  --no-admin --gas-prices 0.025ustars --gas 500000 --gas-adjustment 1.9 \
  --from $KEY -y -b block -o json | jq .
