mod blockchain;
use blockchain::Chain;

fn main() {
    let mut blockchain = Chain::new(String::from("miner_1"), 2);
    
    println!("Adding a new transaction...");
    blockchain.new_transaction(String::from("alice"), String::from("bob"), 50.0);
    
    println!("Blockchain initialized successfully!");
}
