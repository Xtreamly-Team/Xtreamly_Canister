use std::fmt;
use candid::Principal;
use serde::{Deserialize, Serialize};
use ic_cdk::export::candid::CandidType;


#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub(crate) data: String,
    pub(crate) issuer: Principal,
    pub(crate) subject: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct Signature {
    bytes: Vec<u8>,
}


#[derive(Serialize, Deserialize)]
pub enum VerificationPresentationType {
    SELF_PRESENTED,
    VERIFIABLE
}

#[derive(Serialize, Deserialize)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub(crate) context: String,
    #[serde(rename = "type")]
    pub(crate) types: String,
    pub(crate) issuer: Principal,
    pub(crate) issuance_date: String,
    pub(crate) credential_subject: Credential,
    pub(crate) proof: String,
    pub(crate) verification_presentation : VerificationPresentationType,
}

#[derive(Serialize, Deserialize)]
pub struct Proof {
    pub(crate) r#type: String,
    pub(crate) created: String,
    pub(crate) proof_purpose: String,
    pub(crate) verification_method: String,
    pub(crate) proof_value: String,
}





#[derive(Clone)]
pub enum CommandType{
    ERC20_TOKEN_TRANSFER_FROM,
    ERC20_BALANCE
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::ERC20_TOKEN_TRANSFER_FROM => write!(f, "ERC20_TOKEN_TRANSFER"),
            CommandType::ERC20_BALANCE => write!(f, "ERC20_BALANCE")
        }
    }
}

#[derive(Clone)]
pub struct Command {
    pub(crate) command_type: CommandType,
    pub(crate) args: String,
    pub(crate) extra: String
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct KeyHolder {
    pub(crate) derive:Vec<Vec<u8>>,
    pub(crate) proxy_publickey: String,
    pub(crate) actual_publickey: String
}
