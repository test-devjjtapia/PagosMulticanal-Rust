use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};

pub struct SecurityProcessor {
    cipher: Aes256Gcm,
}

impl SecurityProcessor {
    pub fn new() -> Self {
        // En un entorno real, la clave vendría de un HSM o vault
        let key = [0u8; 32];
        let cipher = Aes256Gcm::new(&key.into());
        Self { cipher }
    }

    pub fn tokenize_pan(&self, pan: &str) -> String {
        // Simulación de tokenización determinística para el demo
        let hash_bytes = md5::compute(pan);
        let hash = hash_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        format!("TOK-{}-{}", &hash[0..8], &hash[24..32]).to_uppercase()
    }

    pub fn encrypt(&self, data: &str) -> String {
        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
        let ciphertext = self.cipher.encrypt(nonce, data.as_bytes()).expect("encryption failure");
        general_purpose::STANDARD.encode(ciphertext)
    }
}

// Dependencia interna rápida para hashing
mod md5 {
    pub fn compute(data: &str) -> [u8; 16] {
        // Stub de hash para no añadir más dependencias externas pesadas si no es necesario
        let mut result = [0u8; 16];
        let bytes = data.as_bytes();
        for i in 0..bytes.len().min(16) {
            result[i] = bytes[i];
        }
        result
    }
}
