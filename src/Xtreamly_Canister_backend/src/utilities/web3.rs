use std::str::FromStr;

use base64::decode;
use candid::candid_method;
use hex::encode;
use ic_cdk::{api, export::serde};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::{self, query, update};
use ic_web3::{
    contract::{Contract, Options},
    ethabi::ethereum_types::{U256, U64},
    types::{Address, BlockId, BlockNumber, TransactionParameters},
};
use ic_web3::ic::{get_eth_addr, KeyInfo};
use ic_web3::transports::ICHttp;
use ic_web3::types::Bytes;
use ic_web3::Web3;
use serde::{Deserialize, Serialize};

use crate::KeyHolder;
use crate::utilities::consts::{CHAIN_ID, ERC20_TOKEN_ABI, KEY_NAME, URL};


#[query(name = "transform")]
#[candid_method(query, rename = "transform")]
fn transform(response: TransformArgs) -> HttpResponse {
    response.response
}

// query a contract, token balance
#[update(name = "ERC20_token_balance")]
#[candid_method(update, rename = "ERC20_token_balance")]
pub async fn ERC20_token_balance(contract_addr: String, addr: String) -> Result<String, String> {
    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    let contract_address = Address::from_str(&contract_addr).unwrap();
    let contract = Contract::from_json(
        w3.eth(),
        contract_address,
        ERC20_TOKEN_ABI,
    ).map_err(|e| format!("init contract failed: "))?;

    let addr = Address::from_str(&addr).unwrap();
    let balance: U256 = contract
        .query("balanceOf", (addr, ), None, Options::default(), None)
        .await
        .map_err(|e| format!("query contract error: {}", e))?;
    Ok(format!("{}", balance))
}

// call a contract, transfer some token to addr
#[update(name = "send_ERC20_token_from")]
#[candid_method(update, rename = "send_ERC20_token_from")]
pub async fn send_ERC20_token_from(token_addr: String, from: String, to: String, value: u64, key_holder: String) -> Result<String, String> {
    let key_holder_result: KeyHolder = serde_json::from_str(&String::from_utf8_lossy(&decode(&key_holder).unwrap())).unwrap();

    let derivation_path = key_holder_result.derive;
    let key_info = KeyInfo { derivation_path: derivation_path, key_name: KEY_NAME.to_string(), ecdsa_sign_cycles: None };

    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    let contract_address = Address::from_str(&token_addr).unwrap();
    let contract = Contract::from_json(
        w3.eth(),
        contract_address,
        ERC20_TOKEN_ABI,
    ).map_err(|e| format!("init contract failed: "))?;

    let from = Address::from_str(&from).unwrap();
    // add nonce to options
    let tx_count = w3.eth()
        .transaction_count(from, None)
        .await
        .map_err(|e| format!("get tx count error: {}", e))?;
    // get gas_price
    let gas_price = w3.eth()
        .gas_price()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    // legacy transaction type is still ok
    let options = Options::with(|op| {
        op.nonce = Some(tx_count);
        op.gas_price = Some(gas_price);
        op.transaction_type = Some(U64::from(2)) //EIP1559_TX_ID
    });
    let to_addr = Address::from_str(&to).unwrap();
    let txhash = contract
        .signed_call("transferFrom", (from, to_addr, value, ), options, hex::encode(from), key_info, CHAIN_ID)
        .await
        .map_err(|e| format!("token transfer failed: {}", e))?;

    ic_cdk::println!("txhash: {}", hex::encode(txhash));

    Ok(format!("{}", hex::encode(txhash)))
}

// deploy evm based contract
#[update(name = "deploy_contract")]
#[candid_method(update, rename = "deploy_contract")]
pub async fn deploy_contract(base46_evm_bytecode: String, key_holder: String) -> Result<String, String> {
    let key_holder_result: KeyHolder = serde_json::from_str(&String::from_utf8_lossy(&decode(&key_holder).unwrap())).unwrap();

    let derivation_path = key_holder_result.derive;
    let key_info = KeyInfo { derivation_path: derivation_path, key_name: KEY_NAME.to_string(), ecdsa_sign_cycles: None };

    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    /* let contract_address = Address::from_str(&token_addr).unwrap();
     let contract = Contract::from_json(
         w3.eth(),
         contract_address,
         ERC20_TOKEN_ABI
     ).map_err(|e| format!("init contract failed: "))?;*/

    let from = Address::from_str(&key_holder_result.proxy_publickey).unwrap();
    // add nonce to options
    let tx_count = w3.eth()
        .transaction_count(from, None)
        .await
        .map_err(|e| format!("get tx count error: {}", e))?;
    // get gas_price
    let gas_price = w3.eth()
        .gas_price()
        .await
        .map_err(|e| format!("get gas_price error: {}", e))?;
    // legacy transaction type is still ok
    let options = Options::with(|op| {
        op.nonce = Some(tx_count);
        op.gas_price = Some(gas_price);
        op.transaction_type = Some(U64::from(2)) //EIP1559_TX_ID
    });
    /*    let to_addr = Address::zero();*/
    let tx = TransactionParameters {
        /*   to: Some(to_addr),*/
        nonce: Some(tx_count), // remember to fetch nonce first
        /*value: U256::from(value),*/
        gas_price: Some(U256::exp10(10)), // 10 gwei
        gas: U256::from(21000),
        data: Bytes::from(decode(base46_evm_bytecode).unwrap()),
        ..Default::default()
    };
    let signed_tx = w3.accounts()
        .sign_transaction(tx, hex::encode(from), key_info, CHAIN_ID)
        .await
        .map_err(|e| format!("sign tx error: {}", e))?;
    match w3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
        Ok(txhash) => {
            ic_cdk::println!("txhash: {}", hex::encode(txhash.0));
            Ok(format!("{}", hex::encode(txhash.0)))
        }
        Err(e) => { Err(e.to_string()) }
    }
}

// send tx to eth
#[update(name = "send_eth")]
#[candid_method(update, rename = "send_eth")]
pub async fn send_eth(to: String, value: u64) -> Result<String, String> {
    // ecdsa key info
    let derivation_path = vec![ic_cdk::id().as_slice().to_vec()];
    let key_info = KeyInfo { derivation_path: derivation_path, key_name: KEY_NAME.to_string(), ecdsa_sign_cycles: None };

    // get canister eth address
    let from_addr = get_eth_addr(None, None, KEY_NAME.to_string())
        .await
        .map_err(|e| format!("get canister eth addr failed: {}", e))?;
    // get canister the address tx count
    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    let tx_count = w3.eth()
        .transaction_count(from_addr, None)
        .await
        .map_err(|e| format!("get tx count error: {}", e))?;

    ic_cdk::println!("canister eth address {} tx count: {}", hex::encode(from_addr), tx_count);
    // construct a transaction
    let to = Address::from_str(&to).unwrap();
    let tx = TransactionParameters {
        to: Some(to),
        nonce: Some(tx_count), // remember to fetch nonce first
        value: U256::from(value),
        gas_price: Some(U256::exp10(10)), // 10 gwei
        gas: U256::from(21000),
        ..Default::default()
    };
    // sign the transaction and get serialized transaction + signature
    let signed_tx = w3.accounts()
        .sign_transaction(tx, hex::encode(from_addr), key_info, CHAIN_ID)
        .await
        .map_err(|e| format!("sign tx error: {}", e))?;
    match w3.eth().send_raw_transaction(signed_tx.raw_transaction).await {
        Ok(txhash) => {
            ic_cdk::println!("txhash: {}", hex::encode(txhash.0));
            Ok(format!("{}", hex::encode(txhash.0)))
        }
        Err(e) => { Err(e.to_string()) }
    }
}

// call a contract, transfer some token to addr
#[update(name = "rpc_call")]
#[candid_method(update, rename = "rpc_call")]
pub async fn rpc_call(body: String) -> Result<String, String> {
    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    let res = w3.json_rpc_call(body.as_ref()).await.map_err(|e| format!("{}", e))?;

    ic_cdk::println!("result: {}", res);

    Ok(format!("{}", res))
}

#[update(name = "get_block")]
#[candid_method(update, rename = "get_block")]
pub async fn get_block(number: Option<u64>) -> Result<String, String> {
    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    let block_id = match number {
        Some(id) => { BlockId::from(U64::from(id)) }
        None => { BlockId::Number(BlockNumber::Latest) }
    };
    let block = w3.eth().block(block_id).await.map_err(|e| format!("get block error: {}", e))?;
    ic_cdk::println!("block: {:?}", block.clone().unwrap());

    Ok(serde_json::to_string(&block.unwrap()).unwrap())
}

#[update(name = "get_eth_gas_price")]
#[candid_method(update, rename = "get_eth_gas_price")]
pub async fn get_eth_gas_price() -> Result<String, String> {
    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    let gas_price = w3.eth().gas_price().await.map_err(|e| format!("get gas price failed: {}", e))?;
    ic_cdk::println!("gas price: {}", gas_price);
    Ok(format!("{}", gas_price))
}

// get canister's ethereum address
#[update(name = "get_canister_addr")]
#[candid_method(update, rename = "get_canister_addr")]
pub async fn get_canister_addr() -> Result<String, String> {
    match get_eth_addr(None, None, KEY_NAME.to_string()).await {
        Ok(addr) => { Ok(hex::encode(addr)) }
        Err(e) => { Err(e) }
    }
}

#[update(name = "get_address_with_randomness")]
#[candid_method(update, rename = "get_address_with_randomness")]
pub async fn get_address_with_randomness(seed: Vec<Vec<u8>>) -> Result<String, String> {
    match get_eth_addr(None, Option::from(seed), KEY_NAME.to_string()).await {
        Ok(addr) => { Ok(hex::encode(addr)) }
        Err(e) => { Err(e) }
    }
}

#[update(name = "get_eth_balance")]
#[candid_method(update, rename = "get_eth_balance")]
pub async fn get_eth_balance(addr: String) -> Result<String, String> {
    let w3 = match ICHttp::new(URL, None) {
        Ok(v) => { Web3::new(v) }
        Err(e) => { return Err(e.to_string()); }
    };
    let balance = w3.eth().balance(Address::from_str(&addr).unwrap(), None).await.map_err(|e| format!("get balance failed: {}", e))?;
    Ok(format!("{}", balance))
}
