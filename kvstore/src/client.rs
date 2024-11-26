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

    // // PUT
    // let response = client.put(PutRequest {
    //     key: "key1".into(),
    //     value: "value1".into(),
    // }).await?;
    // println!("PUT Response: {:?}", response);
    //
    // // GET
    // let response = client.get(GetRequest {
    //     key: "key1".into(),
    // }).await?;
    // println!("GET Response: {:?}", response);
    //
    // // DELETE
    // let response = client.delete(DeleteRequest {
    //     key: "key1".into(),
    // }).await?;
    // println!("DELETE Response: {:?}", response);

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
        execute_transaction(client, transaction).await?;
    }

    let duration = start.elapsed();
    let throughput = num_transactions as f64 / duration.as_secs_f64();
    println!(
        "Transactions - Total Time: {:?}, Throughput: {:.2} transactions/sec",
        duration, throughput
    );

    Ok(())
}

async fn execute_transaction(
    client: &mut KvStoreClient<tonic::transport::Channel>,
    transaction: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    for op in transaction {
        match op {
            "GET" => {
                let key = "key1".to_string(); // Example key
                client.get(GetRequest { key }).await?;
            }
            "PUT" => {
                let key = "key1".to_string();
                let value = "value1".to_string();
                client.put(PutRequest { key, value }).await?;
            }
            "DELETE" => {
                let key = "key1".to_string();
                client.delete(DeleteRequest { key }).await?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
