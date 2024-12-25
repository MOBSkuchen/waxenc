use sodiumoxide::crypto::pwhash::{argon2id13};
use sodiumoxide::crypto::hash::hash;
use sodiumoxide::crypto::secretbox;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::display_error;

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

fn encrypt_buffer(password: String, buffer: Vec<u8>) -> bincode::Result<Vec<u8>> {
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

fn decrypt_buffer(password: String, buffer: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let deserialized: EncryptedData = bincode::deserialize(&buffer).unwrap();

    let deserialized_key = derive_key_from_passphrase(password.as_str(), &deserialized.salt)?;

    decrypt(
        &deserialized.ciphertext,
        &secretbox::Nonce::from_slice(&deserialized.nonce).unwrap(),
        &deserialized_key,
    )
}

pub fn encrypt_file_xx(file_name: String, password: String, replace: bool) {
    let mut target_file = file_name.clone() + if replace {""} else {".waxe"};
    let file_buffer_r = fs::read(&file_name);
    if file_buffer_r.is_err() {
        display_error("Operation failed because the file can not be read!".to_string());
        return;
    }
    let file_buffer = file_buffer_r.unwrap();
    let encrypted_r = encrypt_buffer(password, file_buffer);
    if encrypted_r.is_err() {
        display_error("Encryption failed: ".to_string() + encrypted_r.unwrap_err().to_string().as_str());
        return;
    }
    let encrypted_buffer = encrypted_r.unwrap();
    if replace {
        let rm_file_r = fs::remove_file(&target_file);
        if rm_file_r.is_err() {
            display_error(format!("Failed to remove file {}, falling back!", target_file));
            target_file = target_file + ".waxe"
        }
    }
    if fs::exists(&target_file).expect("Why tho") {
        display_error("Target file (".to_owned() + &*target_file + " already exists!");
    }
    let write_r = fs::write(target_file, encrypted_buffer);
    if write_r.is_err() {
        display_error("Could not write encrypted file".to_string());
    }
}

pub fn decrypt_file_xx(file_name: String, password: String, replace: bool) {
    let mut target_file = if replace { file_name.clone() } else { Path::new(&file_name).file_stem().unwrap().to_os_string().into_string().unwrap() + ".waxd" };
    let file_buffer_r = fs::read(&file_name);
    if file_buffer_r.is_err() {
        display_error("Operation failed because the file can not be read!".to_string());
        return;
    }
    let file_buffer = file_buffer_r.unwrap();
    let decrypted_r = decrypt_buffer(password, file_buffer);
    if decrypted_r.is_err() {
        display_error("Decryption failed: ".to_string() + decrypted_r.unwrap_err().to_string().as_str());
        return;
    }
    let decrypted_buffer = decrypted_r.unwrap();
    if replace {
        let rm_file_r = fs::remove_file(&target_file);
        if rm_file_r.is_err() {
            display_error(format!("Failed to remove file {}, falling back! => {}", target_file, rm_file_r.unwrap_err().to_string()));
            target_file = target_file + ".waxd"
        }
    }
    if fs::exists(&target_file).expect("Why tho") {
        display_error("Target file (".to_owned() + &*target_file + ") already exists!");
    }
    let write_r = fs::write(target_file, decrypted_buffer);
    if write_r.is_err() {
        display_error("Could not write decrypted file".to_string());
    }
}

pub fn hash_file(file_name: String, dst_file: OsString) {
    if fs::exists(&dst_file).expect("Why tho") {
        display_error("Target file (".to_owned() + dst_file.to_str().unwrap() + ") already exists!");
    }
    let file_buffer_r = fs::read(&file_name);
    if file_buffer_r.is_err() {
        display_error("Operation failed because the file can not be read!".to_string());
        return;
    }
    let file_buffer = file_buffer_r.unwrap();
    let hashed = hash(&file_buffer);
    let write_r = fs::write(dst_file, hashed);
    if write_r.is_err() {
        display_error("Could not write decrypted file".to_string());
    }
}