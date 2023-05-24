#! /bin/bash
dfx build
rm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm.gz
gzip -f  ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm
dfx canister install Xtreamly_Canister_backend --upgrade-unchanged  --mode=reinstall  --wasm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm.gz -y

