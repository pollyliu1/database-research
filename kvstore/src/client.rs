use kvstore::kv_store_client::KvStoreClient;
use kvstore::{PutRequest, GetRequest, DeleteRequest};
use std::time::{Instant};

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

    let num_ops = 10_000;
    benchmark_put(&mut client, num_ops).await?;
    benchmark_get(&mut client, num_ops).await?;
    benchmark_delete(&mut client, num_ops).await?;

    Ok(())
}


async fn benchmark_put(client: &mut KvStoreClient<tonic::transport::Channel>, num_ops: usize) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    for i in 0..num_ops {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        client.put(PutRequest { key, value }).await?;
    }

    let duration = start.elapsed();
    let throughput = num_ops as f64 / duration.as_secs_f64();
    println!(
        "PUT - Total Time: {:?}, Throughput: {:.2} ops/sec",
        duration, throughput
    );

    Ok(())
}

async fn benchmark_get(client: &mut KvStoreClient<tonic::transport::Channel>, num_ops: usize) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut found = 0;

    for i in 0..num_ops {
        let key = format!("key{}", i);
        let response = client.get(GetRequest { key }).await?;
        if response.into_inner().found {
            found += 1;
        }
    }

    let duration = start.elapsed();
    let throughput = num_ops as f64 / duration.as_secs_f64();
    println!(
        "GET - Total Time: {:?}, Throughput: {:.2} ops/sec, Found: {}",
        duration, throughput, found
    );

    Ok(())
}

async fn benchmark_delete(client: &mut KvStoreClient<tonic::transport::Channel>, num_ops: usize) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    for i in 0..num_ops {
        let key = format!("key{}", i);
        client.delete(DeleteRequest { key }).await?;
    }

    let duration = start.elapsed();
    let throughput = num_ops as f64 / duration.as_secs_f64();
    println!(
        "DELETE - Total Time: {:?}, Throughput: {:.2} ops/sec",
        duration, throughput
    );

    Ok(())
}
