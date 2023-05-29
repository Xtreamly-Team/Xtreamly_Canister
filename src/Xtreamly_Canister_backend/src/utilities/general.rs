
use std::sync::{Arc};
use candid::Principal;
use hex::encode;
use ic_cdk::{api, print, spawn};
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId, sign_with_ecdsa, SignWithEcdsaArgument};
use rhai::{Engine, Scope};
use sha2::{Digest, Sha256};
use crate::models::entities::KeyHolder;
use crate::utilities::rhia_delegation::{icp_time, print_to_icp};


pub async fn create_random_string() -> String {
      let random = api::management_canister::main::raw_rand().await.unwrap().0;
      return hex::encode(&random);
}
pub async fn create_random_derive() -> Vec<Vec<u8>> {
      let random1 = api::management_canister::main::raw_rand().await.unwrap().0;
      let random2 = api::management_canister::main::raw_rand().await.unwrap().0;
      let random3 = api::management_canister::main::raw_rand().await.unwrap().0;
      return vec![random1,random2,random3];
}
pub async fn create_random_did() -> String {
      let random = api::management_canister::main::raw_rand().await.unwrap().0;
      let did = hex::encode(&random);
      did
}
pub fn build_string( stringv : Vec<String>) -> String {
      let mut result = "".to_owned();
      for string in stringv.iter() {
            result.push_str(string);
      }
       return result;
}
pub fn setup_basic_engine(engine: &mut Engine) {

      engine.register_fn("print_to_icp", print_to_icp);
      engine.register_fn("time", icp_time);
}
pub fn setup_basic_scope(scope: &mut Scope, key_holder : KeyHolder) {
      scope.push_constant("MY_PROXY_ACCOUNT",  base64::encode((serde_json::to_string(&key_holder).unwrap())));
      scope.push_constant("MY_PROXY_ADDRESS", key_holder.clone().proxy_publickey);
      scope.push_constant("MY_REAL_ADDRESS", key_holder.clone().actual_publickey);
}
pub fn get_caller_principle() -> Principal {
      let caller = ic_cdk::caller();
      caller
}
pub fn get_time() -> String {
      let time_string = api::time().to_string();
      time_string
}
