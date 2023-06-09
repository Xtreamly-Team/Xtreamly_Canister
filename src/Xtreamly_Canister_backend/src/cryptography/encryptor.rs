use aes::Aes256;
use aes::cipher::{BlockDecrypt, BlockEncrypt, Key, KeyInit};
use hex::{FromHex, ToHex};
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId, sign_with_ecdsa, SignWithEcdsaArgument};
use sha2::{Sha256 ,Digest};

/// encrypts and decrypts a string with a password hard-coded in the canister, ideally this should change but it is good for now
pub fn decrypt_with_password(encrypted: &str) -> String {
    // Convert the password to a fixed-size key
    let mut key = [2u8; 32];
    let iv: [u8; 16] = [1u8; 16];

    let cipher = Aes256::new(<&Key<Aes256>>::from(&key));

    // Decode the encrypted data from hex
    let encrypted_bytes = Vec::from_hex(encrypted).unwrap();

    // Decrypt the data using CBC mode
    let mut decrypted = Vec::new();
    let mut block = iv;
    let mut chunks =  encrypted_bytes.chunks(16);
    while  let Some(item) = chunks.next(){
        let mut i = 0;
        for byte in item.iter() {
            block[i] ^= *byte;
            i += 1;
        }
        cipher.decrypt_block(&mut block.into());
        decrypted.extend_from_slice(&block);
        block.copy_from_slice(item);
    }
    // Remove any trailing null bytes that were added during encryption
    decrypted.retain(|&x| x != 0);
    return String::from_utf8(decrypted).unwrap();
}

pub fn encrypt_with_password(text: &str) -> String {
    // Convert the password to a fixed-size key
    let mut key = [2u8; 32];
    let iv: [u8; 16] = [1u8; 16];

    let cipher = Aes256::new(<&Key<Aes256>>::from(&key));

    // Pad the text to a multiple of the block size
    let block_size = 16;
    let padded_text = match text.len() % block_size {
        0 => text.to_owned(),
        n => format!("{}{}", text, "\0".repeat(block_size - n)),
    };
    // Encrypt the padded text using CBC mode
    let mut encrypted = iv.to_vec();
    let mut block = iv;
    let mut chunks =  padded_text.as_bytes().chunks(block_size);
    while  let Some(item) = chunks.next(){
        let mut i = 0;
        for byte in item.iter() {
            block[i] ^= *byte;
            i = i +1;
        }
        cipher.encrypt_block(&mut block.into());
        encrypted.extend_from_slice(&block);
    }
    return encrypted.encode_hex();
}

pub fn hash_string(input: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let result_string = hex::encode(result);
    return  (&result_string[0..32]).to_owned();
}

pub async fn sign_message(message: String) -> Vec<u8> {
    let message_bytes = hash_string(message.clone()).as_bytes().to_vec().clone();
    //todo : change this to a key generated by the canister id
    let derivation_path: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    let key_id = EcdsaKeyId { curve: EcdsaCurve::Secp256k1, name: "dfx_test_key".to_owned()};
    let args = SignWithEcdsaArgument {
        message_hash: message_bytes,
        derivation_path,
        key_id,
    };
    let signature_entity = sign_with_ecdsa(args).await.unwrap();
    return  signature_entity.0.signature ;
}
