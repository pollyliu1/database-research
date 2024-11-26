use kvstore::kv_store_client::KvStoreClient;
use kvstore::{PutRequest, GetRequest, DeleteRequest};
use std::time::{Instant};
use rand::Rng;
use std::collections::HashMap;

pub mod kvstore {
    tonic::include_proto!("kvstore");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KvStoreClient::connect("http://[::1]:50051").await?;

    let num_transactions = 10_000;

    println!("Benchmarking Read-heavy Transactions...");
    benchmark_transactions(&mut client, num_transactions, 90, 10).await?;

    println!("Benchmarking Write-heavy Transactions...");
    benchmark_transactions(&mut client, num_transactions, 10, 90).await?;

    println!("Benchmarking Mixed Transactions...");
    benchmark_transactions(&mut client, num_transactions, 50, 50).await?;

    Ok(())
}

async fn benchmark_transactions(
    client: &mut KvStoreClient<tonic::transport::Channel>,
    num_transactions: usize,
    read_percentage: u8,
    write_percentage: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let mut data_store = HashMap::new(); // Local data store for verification

    let mut correct_gets = 0;
    let mut total_gets = 0;

    for _ in 0..num_transactions {
        let mut transaction = Vec::new();

        // Generate a transaction with random operations based on the given percentages
        for _ in 0..rng.gen_range(1..10) { // Random number of operations per transaction
            let op_type = rng.gen_range(0..100);
            if op_type < read_percentage {
                transaction.push("GET");
            } else if op_type < read_percentage + write_percentage {
                transaction.push("PUT");
            } else {
                transaction.push("DELETE");
            }
        }

        // Execute the transaction
        let (correct, total) = execute_transaction(client, transaction, &mut data_store).await?;
        correct_gets += correct;
        total_gets += total;
    }

    let duration = start.elapsed();
    let throughput = num_transactions as f64 / duration.as_secs_f64();
    println!(
        "Transactions - Total Time: {:?}, Throughput: {:.2} transactions/sec",
        duration, throughput
    );

    if total_gets > 0 {
        println!(
            "GET Accuracy: {:.2}%",
            (correct_gets as f64 / total_gets as f64) * 100.0
        );
    }

    Ok(())
}

async fn execute_transaction(
    client: &mut KvStoreClient<tonic::transport::Channel>,
    transaction: Vec<&str>,
    data_store: &mut HashMap<String, String>,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut correct_gets = 0;
    let mut total_gets = 0;

    for op in transaction {
        match op {
            "GET" => {
                let key = format!("key{}", rand::thread_rng().gen_range(0..10000));
                let response = client.get(GetRequest { key: key.clone() }).await?;
                let value = response.into_inner().value;

                if let Some(expected_value) = data_store.get(&key) {
                    if *expected_value == value {
                        correct_gets += 1; // Correct GET
                    }
                }
                total_gets += 1; // Total GET attempts
            }
            "PUT" => {
                let key = format!("key{}", rand::thread_rng().gen_range(0..10000));
                let value = format!("value{}", rand::thread_rng().gen_range(0..10000));
                client.put(PutRequest { key: key.clone(), value: value.clone() }).await?;
                data_store.insert(key, value); // Update local store
            }
            "DELETE" => {
                let key = format!("key{}", rand::thread_rng().gen_range(0..10000));
                client.delete(DeleteRequest { key: key.clone() }).await?;
                data_store.remove(&key); // Remove from local store
            }
            _ => unreachable!(),
        }
    }

    Ok((correct_gets, total_gets))
}
