USER=stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd
MINTER=stars1ztgau3w7l89qyd8n7emswjrsf3r7j06y36ngzshk6ky88du9e2jqsyz3u7
KEY=$(starsd keys show $USER | jq -r .name)
MSG=$(cat <<EOF
{
    "mint": {}
}
EOF
)

echo $MSG

starsd tx wasm execute $MINTER "$MSG" --amount '10000000factory/stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd/ufrnz' \
--gas-prices 0.025ustars --gas 10000000 --gas-adjustment 1.9 \
--from stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd -y -b block -o json | jq .