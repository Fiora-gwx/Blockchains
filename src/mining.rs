use crate::block::Block;

// 挖矿策略特质
pub trait MiningStrategy {
    fn mine(&self, block: &mut Block);
}

// 简单的工作量证明挖矿策略
pub struct ProofOfWorkStrategy {
    pub difficulty: usize,
}

impl MiningStrategy for ProofOfWorkStrategy {
    fn mine(&self, block: &mut Block) {
        let target = "0".repeat(self.difficulty);

        while &block.hash[..self.difficulty] != target {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
    }
}

// 随机数挖矿策略
pub struct RandomStrategy;

impl MiningStrategy for RandomStrategy {
    fn mine(&self, block: &mut Block) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        block.nonce = rng.gen_range(1..1000);
        block.hash = block.calculate_hash();
    }
}