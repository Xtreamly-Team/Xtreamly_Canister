use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::rc::Rc;
use std::str::FromStr;
use ic_cdk_macros::{self, update, query};
use ic_web3::contract::{Contract, Options};
use ic_web3::transports::ICHttp;
use ic_web3::types::Address;
use ic_web3::Web3;
use rhai::{Engine, EvalAltResult, Scope};
use crate::utilities::general::{build_string, create_random_derive, create_random_did, create_random_string, get_caller_principle, get_time, setup_basic_engine, setup_basic_scope};
use serde::{Deserialize, Serialize};
use crate::cryptography::encryptor::{decrypt_with_password, encrypt_with_password, sign_message};
use crate::models::entities::{Command, CommandType, Credential, KeyHolder, Proof, VerifiableCredential, VerificationPresentationType};
use crate::utilities::consts::{URL, VC_ABI};
use crate::utilities::web3::{erc20_token_balance, get_address_with_randomness, read_vc_data, send_erc20_token_from};
mod utilities;
mod models;
mod cryptography;


thread_local! {
    static DID_ADDRESS_MAP: RefCell<HashMap<String,String>> = RefCell::new(HashMap::new());
    static DID_PERMISSION_MAP: RefCell<HashMap<String,String>> = RefCell::new(HashMap::new());
    static PROXY_ACCOUNT_HOLDER: RefCell<HashMap<String,KeyHolder>> = RefCell::new(HashMap::new());
    static PROXY_PERMISSION_HOLDER: RefCell<HashMap<String,Vec<String> >> = RefCell::new(HashMap::new());
}




/// get all proxies belong to a public key
#[query]
pub async fn get_proxies(actual_public_key : String)-> String {
    return PROXY_ACCOUNT_HOLDER.with(|map: &RefCell<HashMap<String, KeyHolder>>| {
        let proxy_account_holder = map.borrow();
        let mut found_pairs: Vec<(&String, &KeyHolder)> = Vec::new();
        for (key, value) in proxy_account_holder.iter() {
            if value.actual_publickey == actual_public_key {
                found_pairs.push((key, value));
            }
        }
        return serde_json::to_string(&found_pairs).unwrap();
    });
}

/// create a new proxy account using a real publickey
#[update]
pub async fn create_new_proxy_account(actual_public_key: String) -> String {
    let random_path = create_random_derive().await;
    let token = create_random_string().await;
    let public_key = get_address_with_randomness(random_path.clone()).await.unwrap();
    let key_holder = KeyHolder {
        proxy_publickey : public_key.clone(),
        derive : random_path,
        actual_publickey : actual_public_key
    };
    PROXY_ACCOUNT_HOLDER.with(|map: &RefCell<HashMap<String, KeyHolder>>| (*map).borrow_mut().insert(token.clone(),key_holder.clone() ));

    build_string(vec![token,",".to_owned(), public_key])
}


/// remove a proxy account that is not being used anymore using the token that controls the proxy account
#[update]
pub async fn remove_proxy_account(proxy_account_token: String) -> bool {
    let proxy_account =    PROXY_ACCOUNT_HOLDER.with(|map: &RefCell<HashMap<String, KeyHolder>>| (*map).borrow_mut().remove(&proxy_account_token ));
    match proxy_account {
        Some(proxyaccount) => {
            true
        }
        None => {
            false
        }
    }
}

/// use Rhia language to execute a script using your proxy account
#[ update]
pub async fn execute_script(token: String , stage1_script : String, stage2_string : String ) -> String {
    let stage1_commands: Rc<RefCell<Vec<Command>>> = Rc::new(RefCell::new(Vec::new()));
    let stage2_commands: Rc<RefCell<Vec<Command>>> = Rc::new(RefCell::new(Vec::new()));
    let key_holder =  PROXY_ACCOUNT_HOLDER.with(|map: &RefCell<HashMap<String, KeyHolder>>| (*map).borrow().get(&token).cloned()).unwrap();
    // run stage one script
    let balance_erc20 = {
        let commands = stage1_commands.clone();
        move | variable_name : String  ,contract_address: String, target_address: String| {
            let command = Command {
                command_type: CommandType::ERC20_BALANCE,
                args: build_string(vec![contract_address, ", ".to_owned(), target_address]),
                extra: variable_name,
            };
            commands.borrow_mut().push(command);
        }
    };
    let transfer_erc20 = {
        {
            let commands = stage2_commands.clone();
            move | contract_address : String  ,from: String, to: String , amount : i64 , proxy_account : String| {
                let command = Command {
                    command_type: CommandType::ERC20_TOKEN_TRANSFER_FROM,
                    args: build_string(vec![ contract_address , ", ".to_owned() , from , ", ".to_owned(), to , ", ".to_owned(), amount.to_string() , ", ".to_owned(),proxy_account ]),
                    extra: "".to_owned(),
                };
                commands.borrow_mut().push(command);
            }
        }
    };

    //--------------------
    let mut scope = Scope::new();
    let mut engine = Engine::new();
    setup_basic_scope(&mut scope, key_holder.clone());
    setup_basic_engine(&mut engine);
    //--------------------

    //--------------------
    engine.register_fn("balance_erc20", balance_erc20);
    //--------------------

    let stage1_result : Result<String, Box<EvalAltResult>>   =  engine.eval_with_scope::<String>(&mut scope, &stage1_script);

    // run stage 2 script
    scope = Scope::new();
    engine = Engine::new();
    scope.push_constant("STAGE1_RESULT" , stage1_result.unwrap());
    setup_basic_scope(&mut scope, key_holder.clone());
    setup_basic_engine(&mut engine);
    engine.register_fn("transfer_from_erc20", transfer_erc20);
    for command in stage1_commands.borrow().iter() {
        match command.command_type {
            CommandType::ERC20_BALANCE => {
                let parts: Vec<&str> = command.args.split(',').collect();
                let result = erc20_token_balance(parts[0].parse().unwrap(), parts[1].parse().unwrap()).await.unwrap();
                scope.push_constant(command.clone().extra, result);
            },
            CommandType::ERC20_TOKEN_TRANSFER_FROM => {
                // should not invoke token transfer in stage 1 , so ignore it
            }
        }
    }
    let stage2_result : Result<String, Box<EvalAltResult>>   =  engine.eval_with_scope::<String>(&mut scope ,&stage2_string);
    for command in stage2_commands.borrow().iter() {
        match command.command_type {
            CommandType::ERC20_BALANCE => {
            },
            CommandType::ERC20_TOKEN_TRANSFER_FROM => {
                let parts: Vec<&str> = command.args.split(',').collect();
                let result = send_erc20_token_from(parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap(), parts[3].parse::<u64>().unwrap(), parts[4].parse().unwrap()).await.unwrap();
            }
        }
    }

    return  stage2_result.unwrap().to_string();

}

/// creates  a VC from a claim, sign it with canisters key and send it back to user, it also updates CREATE_VC_CALLBACK as the temporary fix for Metamask snap limitation on calling update methods of the canister
#[update]
pub async fn create_vc_self_presented(data: String) -> String {
    let caller = get_caller_principle();
    let time_string = get_time();
    let encrypted = encrypt_with_password(&*data);
    let credential = Credential {
        data : encrypted,
        issuer: ic_cdk::id(),
        subject: caller,
    };
    let mut vc = VerifiableCredential {
        context: "https://www.w3.org/2018/credentials/v1".to_string(),
        types: "VerifiableCredential".to_string(),
                                                                  issuer: ic_cdk::id(),
                                                                  issuance_date:  time_string.clone(),
                                                                  credential_subject: credential,
                                                                  proof: "".to_string(),
                                                                  verification_presentation : VerificationPresentationType::SELF_PRESENTED
    };
    let did = create_random_did().await;
    let mut proof = Proof {
        r#type: "Secp256k1".to_string(),
        created: time_string.clone(),
        proof_purpose: "assertionMethod".to_string(),
        verification_method: format!("did:xtreamly:{}", did),
        proof_value: "".to_string(),
    };

    let mut vc_json = serde_json::to_string(&vc).unwrap();
    let mut  proof_json = serde_json::to_string(&proof).unwrap();
    let mut  strings = vec![vc_json,proof_json];

    let signature = sign_message(strings.join("")).await;
    proof.proof_value  =  hex::encode(&signature);
    vc.proof = proof.proof_value.clone();
    vc_json = serde_json::to_string(&vc).unwrap();
    proof_json = serde_json::to_string(&proof).unwrap();
    strings = vec![vc_json,proof_json];
    let result = strings.join("\n");
    return result;

}

/// accepting did  and the deployed address of the vc on the EVM based chain and save it to the hashmap, note that it does not violate the stateless claim about Xtreamly as we do not hold the data itself
#[update]
pub async fn present_did_address(did: String , address : String) -> bool {
    // todo : create a status reading function
    DID_ADDRESS_MAP.with(|map: &RefCell<HashMap<String, String>>| (*map).borrow_mut().insert(did.clone(), address.clone()));
    return  true
}


/// read a verifiable credential according to the rules, using the dapp public key and proxy account address
#[update]
pub async fn get_vc(did: String , dapp_publickey :String , proxy_publickey : String) -> String {
    let contract_address =  DID_ADDRESS_MAP.with(|map: &RefCell<HashMap<String, String>>| (*map).borrow().get(&did.clone()).cloned()).unwrap().to_owned();
    let encrypted_data = read_vc_data(contract_address).await;
    let decrypted = decrypt_with_password(&encrypted_data);
    return  decrypted;

}
