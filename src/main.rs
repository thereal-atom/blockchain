use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Hash)]
struct Transaction {
    sender_address: String,
    recipient_address: String,
    amount: u64,
    created_at: DateTime<Utc>,
}

impl Transaction {
    fn new(sender_address: String, recipient_address: String, amount: u64) -> Transaction {
        Transaction {
            sender_address,
            recipient_address,
            amount,
            created_at: Utc::now(),
        }
    }
}

struct Block {
    hash: u64,
    previous_hash: u64,
    transactions: Vec<Transaction>,
}

#[derive(Hash)]
struct PartialBlock<'a> {
    previous_hash:  &'a u64,
    transactions: &'a Vec<Transaction>,
}

fn calculate_block_hash(previous_hash: &u64, transactions: &Vec<Transaction>) -> u64 {
    let data = PartialBlock {
        previous_hash: &previous_hash,
        transactions: transactions,
    };

    let hash = calculate_hash(&data);

    hash
}

impl Block {
    fn new(previous_hash: u64) -> Block {
        // no transactions are present when creating a block so an empty array is used
        let transactions = vec![];

        let hash = calculate_block_hash(&previous_hash, &transactions);

        Block {
            previous_hash,
            transactions,
            hash,
        }
    }

    fn calculate_hash(&mut self) -> u64 {
        let hash = calculate_block_hash(&self.previous_hash, &self.transactions);

        self.hash = hash;

        hash
    }

    fn create_transaction(&mut self, sender_address: String, recipient_address: String, amount: u64) {
        let transaction = Transaction::new(sender_address, recipient_address, amount);

        self.transactions.push(transaction);
        // hash changes every time a transaction is added so it has to be recalculated every time
        self.calculate_hash();
    }
}

struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> BlockChain {
        let genesis_block = Block::new(0);

        BlockChain {
            blocks: vec![genesis_block],
        }
    }
}

fn main() {
    let bc = BlockChain::new();

    println!("{}", bc.blocks[0].hash);
}
