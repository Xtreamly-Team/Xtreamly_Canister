#! /bin/bash
# Make these two commands as option
dfx stop
dfx start --clean --background --host 127.0.0.1:4999
dfx canister create Xtreamly_Canister_backend
dfx build
rm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm.gz
gzip -f  ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm
dfx canister install Xtreamly_Canister_backend --upgrade-unchanged  --mode=reinstall  --wasm ./target/wasm32-unknown-unknown/release/Xtreamly_Canister_backend.wasm.gz -y
