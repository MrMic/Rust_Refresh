use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use time::OffsetDateTime;

//INFO: STRUCTS _________________________________________________________
#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_transactions: Vec<Transaction>,
    difficulty: u32,
    miner_address: String,
    reward: f32,
}

// INFO: IMPLEMENTATIONS ________________________________________________
impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Self {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_transactions: Vec::new(),
            difficulty,
            miner_address,
            reward: 100.0,
        };
        chain.create_genesis_block();
        chain
    }

    // ______________________________________________________________________
    fn create_genesis_block(&mut self) -> bool {
        let mut genesis_block = Block {
            header: Blockheader {
                timestamp: OffsetDateTime::now_utc().unix_timestamp(),
                nonce: 0,
                pre_hash: self.last_hash(),
                merkle: String::new(),
                difficulty: self.difficulty,
            },
            count: 0,
            transactions: vec![],
        };

        let reward_transaction = Transaction {
            sender: String::from("root"),
            receiver: self.miner_address.clone(),
            amount: self.reward,
        };

        genesis_block.transactions.push(reward_transaction);
        genesis_block
            .transactions
            .append(&mut self.curr_transactions);
        genesis_block.count = genesis_block.transactions.len() as u32;
        genesis_block.header.merkle = Chain::get_merkle(genesis_block.transactions.clone());
        Chain::proof_of_work(&mut genesis_block.header);

        println!("Genesis Block Created: {:#?}", &genesis_block);
        self.chain.push(genesis_block);

        true
    }

    // ______________________________________________________________________
    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
        };
        self.curr_transactions.push(transaction);
        true
    }

    // ______________________________________________________________________
    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    // ______________________________________________________________________
    pub fn update_difficulty(&mut self, new_difficulty: u32) -> bool {
        self.difficulty = new_difficulty;
        true
    }

    // ______________________________________________________________________
    pub fn update_reward(&mut self, new_reward: f32) -> bool {
        self.reward = new_reward;
        true
    }

    // ______________________________________________________________________
    fn get_merkle(curr_transactions: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();
        for t in &curr_transactions {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let h2 = merkle.remove(0);
            h1.push_str(&h2);
            let new_hash = Chain::hash(&h1);
            merkle.push(new_hash);
        }
        merkle.pop().unwrap()
    }

    // ______________________________________________________________________
    fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..(header.difficulty as usize)];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val == 0 {
                        println!("Proof of work found - Block hash: {}", hash);
                        break;
                    } else {
                        header.nonce += 1;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    // ______________________________________________________________________
    fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(item).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    // ______________________________________________________________________
    fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for byte in vec_res {
            write!(&mut s, "{:02x}", byte).expect("Unable to write");
        }
        s
    }
}
