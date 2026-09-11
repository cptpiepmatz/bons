use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use sha2::{Digest, Sha256};

pub struct Key([u8; 32]);

impl Key {
    pub fn random() -> Self {
        Self(rand::random())
    }

    pub fn as_base64(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.0)
    }

    pub fn from_base64(key: impl AsRef<str>) -> Option<Self> {
        let bytes = BASE64_URL_SAFE_NO_PAD.decode(key.as_ref()).ok()?;
        Some(Self(bytes.try_into().ok()?))
    }

    pub fn hash(&self) -> Vec<u8> {
        Sha256::digest(self.0).to_vec()
    }
}

pub struct SessionToken([u8; 32]);

impl SessionToken {
    pub fn random() -> Self {
        Self(rand::random())
    }

    pub fn as_base64(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(self.0)
    }

    pub fn from_base64(token: impl AsRef<str>) -> Option<Self> {
        let bytes = BASE64_URL_SAFE_NO_PAD.decode(token.as_ref()).ok()?;
        Some(Self(bytes.try_into().ok()?))
    }

    pub fn hash(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(Sha256::digest(self.0))
    }
}
