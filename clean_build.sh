#! /bin/bash
# Usually 4943
icp_port="$1"
ganache_host="$2"
ganache_networkId="$3"

dfx stop
dfx start --clean --background --host "127.0.0.1:$icp_port"
dfx canister create Xtreamly_Canister_backend

./change_network.sh "$ganache_host" "$ganache_networkId"

dfx build
rm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm.gz
gzip -f  ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm
dfx canister install Xtreamly_Canister_backend --upgrade-unchanged  --mode=reinstall  --wasm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm.gz -y
