MSG=$(cat <<EOF
{
	"num_tokens": 100,
	"start_time": "1680557400000000000",
	"per_address_limit": 5,
	"base_token_uri": "ipfs://QmYxw1rURvnbQbBRTfmVaZtxSrkrfsbodNzibgBrVrUrtN",
	"sg721_code_id": 1972,
    "unit_price": {
		"denom": "ustars",
        "amount": "100000000"
    },
	"sg721_instantiate_msg": {
		"name": "TEST",
		"symbol": "TEST",
		"minter": "stars1cfudsnwnfezvqjnlhtxhssvzneykysc89ad3nm",
		"collection_info": {
			"creator": "stars1cfudsnwnfezvqjnlhtxhssvzneykysc89ad3nm",
			"description": "Stargaze Monkeys",
			"image": "https://example.com/image.png",
			"royalty_info": {
				"payment_address": "stars1cfudsnwnfezvqjnlhtxhssvzneykysc89ad3nm",
				"share": "0.05"
			}
		}
	}
}
)

starsd tx wasm instantiate 1973 "$MSG" --label "Minter" --amount 1000000000ustars \
    --admin stars1cfudsnwnfezvqjnlhtxhssvzneykysc89ad3nm \
    --gas-prices 0.025ustars --gas auto --gas-adjustment 1.9 \
    --from stars1cfudsnwnfezvqjnlhtxhssvzneykysc89ad3nm -y -b block -o json | jq .
