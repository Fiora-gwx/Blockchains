
use std::collections::HashMap;
use crate::block::Block;
use crate::mining::{MiningStrategy, ProofOfWorkStrategy};

#[derive(Debug)]
pub enum BlockchainError {
    InvalidBlock,
    BlockNotFound,
    ChainBroken,
}

pub struct Blockchain<T: MiningStrategy> {
    pub chain: Vec<Block>,
    pub mining_strategy: T,
    pub balances: HashMap<String, u64>,
    pub pending_transactions: Vec<String>,
}

impl<T: MiningStrategy> Blockchain<T> {
    pub fn new(mining_strategy: T) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            mining_strategy,
            balances: HashMap::new(),
            pending_transactions: Vec::new(),
        };

        // 添加创世区块
        blockchain.create_genesis_block();

        // 初始化一些账户余额
        blockchain.balances.insert(String::from("miner"), 100);

        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::genesis();
        self.chain.push(genesis_block);
        println!("创世区块已生成！");
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn add_block(&mut self, data: String) -> Result<(), BlockchainError> {
        if let Some(latest_block) = self.get_latest_block() {
            let new_index = latest_block.index + 1;
            let new_timestamp = chrono::Utc::now().timestamp();
            let new_prev_hash = latest_block.hash.clone();

            let mut new_block = Block::new(
                new_index,
                new_timestamp,
                new_prev_hash,
                data,
            );

            // 挖掘新区块
            new_block.mine_block(&self.mining_strategy);

            // 验证新区块
            if !new_block.is_valid() {
                return Err(BlockchainError::InvalidBlock);
            }

            self.chain.push(new_block);

            if let Some(balance) = self.balances.get_mut("miner") {
                *balance += 10; // 矿工奖励
            }

            Ok(())
        } else {
            Err(BlockchainError::ChainBroken)
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        // 首先检查创世区块是否有效
        if !self.chain[0].is_valid() {
            println!("创世区块无效");
            return false;
        }

        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if !current_block.is_valid() {
                println!("区块 {} 无效", i);
                return false;
            }

            if current_block.prev_hash != previous_block.hash {
                println!("区块 {} 的前一个哈希与区块 {} 的哈希不匹配", i, i-1);
                println!("区块 {} 的前一个哈希: {}", i, current_block.prev_hash);
                println!("区块 {} 的哈希: {}", i-1, previous_block.hash);
                return false;
            }
        }

        true
    }

    // 添加待处理交易
    pub fn add_transaction(&mut self, transaction: String) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self) -> Result<(), BlockchainError> {
        if self.pending_transactions.is_empty() {
            return Err(BlockchainError::InvalidBlock);
        }

        // 克隆交易列表，以避免借用冲突
        let transactions = self.pending_transactions.join("|");

        // 在添加区块之前清空待处理交易
        self.pending_transactions.clear();

        // 添加包含交易的区块
        self.add_block(transactions)
    }


    pub fn get_block_by_index(&self, index: u64) -> Option<&Block> {
        self.chain.iter().find(|block| block.index == index)
    }


    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.chain.iter().find(|block| block.hash == hash)
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn get_latest_block_owned(&self) -> Option<Block> {
        self.chain.last().cloned()
    }
}