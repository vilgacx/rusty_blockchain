use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

struct Blockchain {
    chain: Vec<String>,
}

struct Block {
    prev: String,
    hash: String,
    nonce: i32,
    transactions: Vec<String>,
}

struct Transaction {
    sender: String,
    reciever: String,
    amount: i32,
}

impl Block {
    fn calc_hash(&self) -> String {
        let mut hashtx: String = "".to_owned();
        for transaction in (&self.transactions).iter() {
            hashtx.push_str(&transaction.to_owned());
        }
        let hashstr = format!(
            "{}{}{}{}{}",
            epoch(),
            &self.nonce,
            hashtx,
            &self.prev,
            &self.hash
        );
        return digest(hashstr);
    }
}

impl Transaction {
    fn calc_hash(&self) -> String {
        let hashstr = format!(
            "{}{}{}{}",
            &self.sender,
            &self.reciever,
            &self.amount.to_string(),
            epoch()
        );
        return digest(hashstr);
    }
}

fn epoch() -> String {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time error")
        .as_millis()
        .to_string();
}

fn main() {
    let test_tx1 = Transaction {
        sender: "dsdsd".to_string(),
        reciever: "dsdsdsd".to_string(),
        amount: 100,
    };
    let test_tx2 = Transaction {
        sender: "4kfsf".to_string(),
        reciever: "yy3q3sd".to_string(),
        amount: 120,
    };

    let gg = Block {
        prev: "xddd".to_string(),
        hash: "dsdsa".to_string(),
        nonce: 0,
        transactions: vec![test_tx1.calc_hash(), test_tx2.calc_hash()],
    };
    println!("{}", gg.calc_hash());
}
