use chrono::{DateTime, Utc};
use sha256::{digest};

struct Transaction {
    sender_address: String,
    recipient_address: String,
    amount: f64,
    created_at: DateTime<Utc>,
}

impl Transaction {
    fn new(sender_address: String, recipient_address: String, amount: f64) -> Transaction {
        Transaction {
            sender_address,
            recipient_address,
            amount,
            created_at: Utc::now(),
        }
    }

    fn to_string(&self) -> String {
        let v = vec![
            self.sender_address.to_owned(),
            self.recipient_address.to_owned(),
            self.amount.to_string(),
            // self.created_at.to_string(),
        ];

        v.join("-")
    }
}

struct Block {
    index: usize,
    hash: String,
    previous_hash: String,
    transactions: Vec<Transaction>,
}

struct PartialBlock<'a> {
    previous_hash:  &'a String,
    transactions: &'a Vec<Transaction>,
}

fn calculate_block_hash(previous_hash: &String, transactions: &Vec<Transaction>) -> String {
    let mut data = String::from(previous_hash);

    data.push_str("-");

    for transaction in transactions {
        data.push_str(&transaction.to_string().to_owned());
        data.push_str("-");
    }

    digest(data)
}

impl Block {
    fn new(index: usize, previous_hash: String, transactions: Vec<Transaction>) -> Block {
        let hash = calculate_block_hash(&previous_hash, &transactions);

        Block {
            index,
            previous_hash,
            transactions,
            hash,
        }
    }

    fn calculate_hash(&mut self) {
        let hash = calculate_block_hash(&self.previous_hash, &self.transactions);

        self.hash = hash
    }

    fn create_transaction(&mut self, sender_address: String, recipient_address: String, amount: f64) {
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
    fn new(&self) -> BlockChain {
        let genesis_block = Block::new(self.blocks.len(), String::from("genesis"), vec![]);

        BlockChain {
            blocks: vec![genesis_block],
        }
    }
}

struct Wallet {
    address: String,
    transactions: Vec<Transaction>,
}

impl Wallet {
    fn new() -> Wallet {
        Wallet {
            address: String::from("abcdefg"),
            transactions: vec![],
        }
    }

    fn create_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }
}

fn main() {
    // let w1 = Wallet::new();
    // let w2 = Wallet::new();
    // let w3 = Wallet::new();
    // let w4 = Wallet::new();
    // let w5 = Wallet::new();

    let t1 = Transaction::new(String::from("w1"), String::from("w2"), 1.0);
    let t2 = Transaction::new(String::from("w2"), String::from("w1"), 2.0);
    let t3 = Transaction::new(String::from("w4"), String::from("w2"), 1.4);
    let t4 = Transaction::new(String::from("w5"), String::from("w1"), 1.8);
    let t5 = Transaction::new(String::from("w2"), String::from("w3"), 2.3);
    let t6 = Transaction::new(String::from("w5"), String::from("w4"), 0.5);
    let t7 = Transaction::new(String::from("w2"), String::from("w5"), 1.6);

    let genesis = Block::new(0, String::from(""), vec![]);
    let b1 = Block::new(1, genesis.hash, vec![t1, t2, t3]);
    let b2 = Block::new(2, b1.hash, vec![t4, t5]);
    let b3 = Block::new(3, b2.hash, vec![t6, t7]);

    println!("{}", b3.hash);
}
