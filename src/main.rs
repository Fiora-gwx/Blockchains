use Blockchains::{
    blockchain::Blockchain,
    mining::ProofOfWorkStrategy,
};

fn main() {
    // 使用工作量证明挖矿策略，难度为2
    let pow_strategy = ProofOfWorkStrategy { difficulty: 2 };
    let mut blockchain = Blockchain::new(pow_strategy);

    // 显示创世区块
    if let Some(genesis_block) = blockchain.get_latest_block() {
        println!("\n创世区块信息：");
        println!("索引: {}", genesis_block.index);
        println!("时间戳: {}", genesis_block.timestamp);
        println!("哈希值: {}", genesis_block.hash);
        println!("前一个哈希: {}", genesis_block.prev_hash);
        println!("数据: {}", genesis_block.data);
        println!("Nonce: {}", genesis_block.nonce);
        println!("默克尔根: {}", genesis_block.merkle_root);
    }

    // 添加交易到待处理池
    blockchain.add_transaction(String::from("Alice 向 Bob 转账 5 个币"));
    blockchain.add_transaction(String::from("Bob 向 Charlie 转账 2 个币"));

    // 挖掘包含这些交易的新区块
    println!("\n挖掘包含待处理交易的区块...");
    match blockchain.mine_pending_transactions() {
        Ok(_) => {
            if let Some(block) = blockchain.get_latest_block() {
                println!("新区块已添加: {:?}", block);
            }
        },
        Err(e) => println!("添加区块失败: {:?}", e),
    }

    // 添加另一个区块
    println!("\n添加另一个区块...");
    match blockchain.add_block(String::from("直接添加的区块数据")) {
        Ok(_) => {
            if let Some(block) = blockchain.get_latest_block() {
                println!("新区块已添加: {:?}", block);
            }
        },
        Err(e) => println!("添加区块失败: {:?}", e),
    }

    // 验证区块链
    if blockchain.is_chain_valid() {
        println!("\n区块链验证成功！");
    } else {
        println!("\n区块链验证失败！");
    }

    // 显示矿工余额
    println!("\n矿工余额: {}", blockchain.get_balance("miner"));

    // 打印整个区块链
    println!("\n完整区块链：");
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("区块 #{}: {:?}", i, block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Blockchains::{
        block::Block,
        blockchain::Blockchain,
        mining::{ProofOfWorkStrategy, RandomStrategy},
    };

    #[test]
    fn test_genesis_block_creation() {
        let strategy = RandomStrategy;
        let blockchain = Blockchain::new(strategy);

        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].prev_hash, "0");
    }

    #[test]
    fn test_block_validation() {
        let mut block = Block::new(1, 1000, "prev_hash".to_string(), "test data".to_string());
        let original_hash = block.hash.clone();

        assert!(block.is_valid());

        // 篡改数据
        block.data = "tampered data".to_string();
        assert!(!block.is_valid());

        // 重新计算哈希
        block.hash = block.calculate_hash();
        assert!(block.is_valid());
    }

    #[test]
    fn test_blockchain_validity() {
        let strategy = RandomStrategy;
        let mut blockchain = Blockchain::new(strategy);

        // 添加有效区块
        blockchain.add_block("test data".to_string()).unwrap();
        assert!(blockchain.is_chain_valid());
        // 篡改区块链
        blockchain.chain[0].data = "tampered data".to_string();
        assert!(!blockchain.is_chain_valid());

    }

    #[test]
    fn test_mining_strategies() {
        // 工作量证明策略
        let pow_strategy = ProofOfWorkStrategy { difficulty: 1 };
        let mut block1 = Block::new(1, 1000, "prev".to_string(), "data".to_string());
        block1.mine_block(&pow_strategy);
        assert_eq!(&block1.hash[0..1], "0");

        // 随机策略
        let random_strategy = RandomStrategy;
        let mut block2 = Block::new(1, 1000, "prev".to_string(), "data".to_string());
        block2.mine_block(&random_strategy);
        assert!(block2.is_valid());
    }

    #[test]
    fn test_transactions_and_balances() {
        let strategy = RandomStrategy;
        let mut blockchain = Blockchain::new(strategy);

        assert_eq!(blockchain.get_balance("miner"), 100);

        blockchain.add_transaction("transaction1".to_string());
        blockchain.add_transaction("transaction2".to_string());

        blockchain.mine_pending_transactions().unwrap();

        // 检查矿工余额是否增加
        assert_eq!(blockchain.get_balance("miner"), 110);

        // 检查交易池是否清空
        assert!(blockchain.pending_transactions.is_empty());
    }
}