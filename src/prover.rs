use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use serde::{Serialize, Deserialize};
use serde_json;
use sha2::{Digest, Sha256};
use hex::encode;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn main() {
    let listener = TcpListener::bind("localhost:7878").expect("Could not bind to port 7878");
    println!("Operator listening on port 7878");

    loop {
        // Accepting transactions from users
        let (user_stream, _) = listener.accept().expect("Failed to accept user connection");
        println!("User connected");

        let mut user_reader = BufReader::new(user_stream);
        let mut transactions = Vec::new();
        let mut line = String::new();

        // Read transactions from the user
        while user_reader.read_line(&mut line).expect("Failed to read from user") > 0 {
            if let Ok(tx) = serde_json::from_str::<Transaction>(&line) {
                transactions.push(tx);
            }
            line.clear();
        }

        if !transactions.is_empty() {
            // Process transactions and generate proof
            let transaction_hashes = generate_transaction_hashes(&transactions);
            let proof = generate_merkle_root(&transaction_hashes);

            // Connect to the verifier and send proof along with transaction hashes
            let mut verifier_stream = TcpStream::connect("localhost:7879").expect("Could not connect to verifier");
            let proof_data = serde_json::json!({
                "transaction_summary": transaction_hashes,
                "proof": proof
            });

            let serialized = serde_json::to_string(&proof_data).unwrap();
            verifier_stream.write_all(serialized.as_bytes()).expect("Failed to write to verifier stream");
            verifier_stream.write_all(b"\n").expect("Failed to write newline to verifier stream");
            println!("Proof and transaction hashes sent to verifier.");

            // Listen for a challenge from the verifier and respond
            let mut verifier_reader = BufReader::new(verifier_stream);
            let mut challenge = String::new();
            verifier_reader.read_line(&mut challenge).expect("Failed to read challenge from verifier");
            let challenge: usize = challenge.trim().parse().expect("Failed to parse challenge");

            if challenge < transaction_hashes.len() {
                println!("Sending response hash: {}", transaction_hashes[challenge]);
                verifier_reader.get_mut().write_all(transaction_hashes[challenge].as_bytes()).expect("Failed to respond to challenge");
                verifier_reader.get_mut().write_all(b"\n").expect("Failed to write newline after challenge response");
            }
        }
    }
}

fn generate_transaction_hashes(transactions: &[Transaction]) -> Vec<String> {
    transactions.iter().map(|tx| {
        let tx_json = serde_json::to_string(tx).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(tx_json.as_bytes());
        encode(hasher.finalize())
    }).collect()
}

fn generate_merkle_root(transaction_hashes: &[String]) -> String {
    let concatenated_hashes = transaction_hashes.concat();
    let mut hasher = Sha256::new();
    hasher.update(concatenated_hashes.as_bytes());
    encode(hasher.finalize())
}
