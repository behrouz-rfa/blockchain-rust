use crate::errors::Result;
use bitcoincash_addr::Address;
use log::debug;
use serde::{Serialize, Deserialize};
use crate::wallet::hash_pub_key;

/// TXInput represents a transaction input
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

/// TXOutput represents a transaction output
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

impl TXInput {
    /// CanUnlockOutputWith checks whether the address initiated the transaction
    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        let mut pubkeyhash = self.pub_key.clone();
        hash_pub_key(&mut pubkeyhash);
        pubkeyhash == pub_key_hash
    }
}

impl TXOutput {
    /// CanBeUnlockedWith checks if the output can be unlocked with the provided data
    pub fn is_locked_with_key(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash == pub_key_hash
    }

    pub fn new(value: i32, address: String) -> Result<Self> {
        let mut txo = TXOutput {
            value,
            pub_key_hash: Vec::new(),
        };
        txo.lock(&address);
        Ok(txo)
    }

    fn lock(&mut self, address: &str) -> Result<()> {
        let pub_key_hash = Address::decode(address).unwrap().body;
        debug!("lock,{}",address);
        self.pub_key_hash = pub_key_hash;
        Ok(())
    }
}
