extern crate bcrypt;

use age::{DecryptError, EncryptError};
use age::scrypt::{Identity, Recipient};
use age::secrecy::SecretString;

fn craft_identity(s: String) -> Identity {
    Identity::new(SecretString::from(s))
}

fn craft_recipient(s: String) -> Recipient {
    Recipient::new(SecretString::from(s))
}

pub fn encrypt_buffer(buffer: Vec<u8>, password: String) -> Result<Vec<u8>, EncryptError> {
    age::encrypt(&craft_recipient(password), buffer.as_ref())
}

pub fn decrypt_buffer(encrypted: Vec<u8>, password: String) -> Result<Vec<u8>, DecryptError> {
    age::decrypt(&craft_identity(password), &encrypted)
}
