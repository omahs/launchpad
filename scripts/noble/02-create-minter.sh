KEY=$(starsd keys show $ADMIN | jq -r .name)
FACTORY=stars1ky6q3xrz6wr38csdaa7c09aeglptf7pawvsrdqyl96zhpyetwerqmsxj2u
SG721_CODE_ID=2784

# add a few minutes buffer to start time
TIME=$(date -v+5S +%s)
END_TIME=$(date -v+5000000S +%s)

MSG=$(cat <<EOF
{
    "create_minter": {
        "init_msg": {
            "nft_data": {
                "nft_data_type": "off_chain_metadata",
                "token_uri": "ipfs://bafybeiavall5udkxkdtdm4djezoxrmfc6o5fn2ug3ymrlvibvwmwydgrkm/1.jpg"
            },
            "start_time": "$(echo $TIME)000000000",
            "end_time": "$(echo $END_TIME)000000000",
            "mint_price": { "amount": "10000000", "denom": "factory/stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd/ufrnz" },
            "per_address_limit": 30
        },
        "collection_params": {
            "code_id": $SG721_CODE_ID,
            "name": "Test Collection noble",
            "symbol": "FRNZ",
            "info": {
                "creator": "$ADMIN",
                "description": "Test Collection noble",
                "image": "ipfs://bafybeiavall5udkxkdtdm4djezoxrmfc6o5fn2ug3ymrlvibvwmwydgrkm/1.jpg"
            }
        }
    }
}
EOF
)

echo $MSG

starsd tx wasm execute $FACTORY "$MSG" --amount 1000000000ustars \
--gas-prices 0.025ustars --gas 10000000 --gas-adjustment 1.9 \
--from $KEY -y -b block -o json | jq .