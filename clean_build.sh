#! /bin/bash
# Make these two commands as option
dfx stop
dfx start --clean --background --host 127.0.0.1:4999
dfx canister create ProxyManager_backend
dfx build
rm ./target/wasm32-unknown-unknown/release/ProxyManager_backend.wasm.gz
gzip -f  ./target/wasm32-unknown-unknown/release/ProxyManager_backend.wasm
dfx canister install ProxyManager_backend --upgrade-unchanged  --mode=reinstall  --wasm ./target/wasm32-unknown-unknown/release/ProxyManager_backend.wasm.gz -y
