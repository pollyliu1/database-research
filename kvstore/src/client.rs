use kvstore::kv_store_client::KvStoreClient;
use kvstore::{PutRequest, GetRequest, DeleteRequest};

pub mod kvstore {
    tonic::include_proto!("kvstore");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KvStoreClient::connect("http://[::1]:50051").await?;

    // PUT
    let response = client.put(PutRequest {
        key: "key1".into(),
        value: "value1".into(),
    }).await?;
    println!("PUT Response: {:?}", response);

    // GET
    let response = client.get(GetRequest {
        key: "key1".into(),
    }).await?;
    println!("GET Response: {:?}", response);

    // DELETE
    let response = client.delete(DeleteRequest {
        key: "key1".into(),
    }).await?;
    println!("DELETE Response: {:?}", response);

    Ok(())
}
