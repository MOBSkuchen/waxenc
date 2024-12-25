use sodiumoxide::crypto::pwhash::{argon2id13};
use sodiumoxide::crypto::secretbox;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct EncryptedData {
    ciphertext: Vec<u8>,
    nonce: [u8; secretbox::NONCEBYTES],
    salt: [u8; argon2id13::SALTBYTES],
}

fn derive_key_from_passphrase(passphrase: &str, salt: &[u8]) -> Result<secretbox::Key, Box<dyn Error>> {
    let mut key = secretbox::Key([0u8; secretbox::KEYBYTES]);
    argon2id13::derive_key(
        &mut key.0,
        passphrase.as_bytes(),
        &argon2id13::Salt(salt.try_into().unwrap()),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    ).unwrap();
    Ok(key)
}

fn encrypt(plaintext: &[u8], key: &secretbox::Key) -> (Vec<u8>, secretbox::Nonce) {
    let nonce = secretbox::gen_nonce();
    let ciphertext = secretbox::seal(plaintext, &nonce, key);
    (ciphertext, nonce)
}

fn decrypt(ciphertext: &[u8], nonce: &secretbox::Nonce, key: &secretbox::Key) -> Result<Vec<u8>, Box<dyn Error>> {
    let plaintext = secretbox::open(ciphertext, nonce, key).unwrap();
    Ok(plaintext)
}

pub fn encrypt_buffer(password: String, buffer: Vec<u8>) -> bincode::Result<Vec<u8>> {
    let salt = argon2id13::gen_salt();
    let key = derive_key_from_passphrase(password.as_str(), (&salt).as_ref()).unwrap();
    let (ciphertext, nonce) = encrypt(&*buffer, &key);

    let encrypted_data = EncryptedData {
        ciphertext,
        nonce: nonce.0,
        salt: salt.0,
    };

    bincode::serialize(&encrypted_data)
}

pub fn decrypt_buffer(password: String, buffer: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let deserialized: EncryptedData = bincode::deserialize(&buffer).unwrap();

    let deserialized_key = derive_key_from_passphrase(password.as_str(), &deserialized.salt)?;

    decrypt(
        &deserialized.ciphertext,
        &secretbox::Nonce::from_slice(&deserialized.nonce).unwrap(),
        &deserialized_key,
    )
}