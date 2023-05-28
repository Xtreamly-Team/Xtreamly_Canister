#! /bin/bash
host="\"https:\/\/$1\""
chainId="$2"

echo "$host"
echo "$chainId"

sed_command="s/const URL: &str = .+\$/const URL: \&str = $host;/g; s/const CHAIN_ID: u64 = .+\$/const CHAIN_ID: u64 = $chainId;/g;" 

echo "Substituting using below sed command"
echo "$sed_command"

sed -E "$sed_command" "./src/Xtreamly_Canister_backend/src/utilities/consts.rs" > changed_consts.rs

rm './src/Xtreamly_Canister_backend/src/utilities/consts.rs' 
mv changed_consts.rs './src/Xtreamly_Canister_backend/src/utilities/consts.rs'
