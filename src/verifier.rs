use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use serde_json::Value;
use rand::{thread_rng, Rng};

fn main() {
    let listener = TcpListener::bind("localhost:7879").expect("Could not bind to port 7879");
    println!("Mainnet listening on port 7879");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to accept incoming connection: {}", e);
                continue;
            }
        };
        println!("Operator connected");

        let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
        let mut proof_data_string = String::new();
        if reader.read_line(&mut proof_data_string).is_err() {
            eprintln!("Failed to read from stream");
            continue;
        }

        let proof_data: Value = match serde_json::from_str(&proof_data_string) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse proof data: {}", e);
                continue;
            }
        };

        let transaction_summary: Vec<String> = match proof_data["transaction_summary"].as_array() {
            Some(summary) => summary.iter().map(|s| s.to_string()).collect(),
            None => {
                eprintln!("Transaction summary is not an array");
                continue;
            }
        };

        let proof: String = match proof_data["proof"].as_str() {
            Some(p) => p.to_owned(),
            None => {
                eprintln!("Proof is not a string");
                continue;
            }
        };

        println!("Received transaction hashes:");
        for hash in &transaction_summary {
            println!("{}", hash);
        }
        println!("Received proof (Merkle Root): {}", proof);

        let challenge = thread_rng().gen_range(0..transaction_summary.len());
        if let Err(e) = write!(stream, "{}\n", challenge) {
            eprintln!("Failed to send challenge to operator: {}", e);
            continue;
        }

        let mut response = String::new();
        if reader.read_line(&mut response).is_err() {
            eprintln!("Failed to read response from operator");
            continue;
        }
        let response = response.trim_end();
        let expected_hash = transaction_summary[challenge].trim_matches('"'); // Strip quotation marks
        println!("Stripped expected hash: {}", expected_hash);

        let verified = expected_hash == response.trim();

        if verified {
            println!("Verification Success: Transactions committed to the blockchain.");
        } else {
            println!("Verification Failed: Invalid proof. Transactions not committed.");
        }
    }
}
