use chrono::Utc;
use sha2::{Sha256, Digest};
use std::fmt::{self, Debug, Formatter};
use crate::mining::MiningStrategy;

#[derive(Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub hash: String,
    pub prev_hash: String,
    pub data: String,
    pub nonce: u64,
    pub merkle_root: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block {{ index: {}, hash: {}, prev_hash: {}, timestamp: {}, data: {}, nonce: {}, merkle_root: {} }}",
               self.index, self.hash, self.prev_hash, self.timestamp, self.data, self.nonce, self.merkle_root)
    }
}

impl Block {
    pub fn new(index: u64, timestamp: i64, prev_hash: String, data: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            hash: String::new(),
            prev_hash,
            data,
            nonce: 0,
            merkle_root: String::from("0"),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let contents = format!("{}{}{}{}{}{}",
                               self.index,
                               self.timestamp,
                               self.prev_hash,
                               self.data,
                               self.nonce,
                               self.merkle_root
        );

        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    pub fn mine_block<T: MiningStrategy>(&mut self, mining_strategy: &T) {
        mining_strategy.mine(self);
        println!("Block mined: {}", self.hash);
    }


    pub fn genesis() -> Self {
        let mut genesis = Block::new(
            0,
            Utc::now().timestamp(),
            String::from("0"),
            String::from("Genesis Block"),
        );
        genesis.merkle_root = genesis.calculate_merkle_root();
        genesis.hash = genesis.calculate_hash();
        genesis
    }

    fn calculate_merkle_root(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // 验证区块是否有效
    pub fn is_valid(&self) -> bool {
        &self.hash == &self.calculate_hash()
    }
}