USER=stars10w5eulj60qp3cfqa0hkmke78qdy2feq6x9xdmd
MINTER=stars1hpg499cgknzv7er5vkrl47a8t9fm9u2ttfn7earvgw5lkyscm82qzd46mh
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