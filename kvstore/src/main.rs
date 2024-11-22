use rust_rocksdb::{DB, Options};
use tonic::transport::Server;
use server::KVStoreService;
use server::kvstore::kv_store_server::KvStoreServer; // for gRPC service
use tokio::sync::mpsc;
use tokio::task;
use router::{Router, LoadBalancer};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

mod server;
mod router;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "3")]
    num_servers: usize,

    #[clap(short, long, default_value = "50051")]
    base_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // spawn servers
    let mut handles = Vec::new();
    let mut server_addrs = Vec::new();
    for i in 0..args.num_servers {
        let port = args.base_port + i as u16;
        let addr = format!("[::1]:{}", port);
        server_addrs.push(addr.clone());
        println!("Starting server on {}", addr);
    
        let service = KVStoreService::new(5, port); // Pass the port as instance_id
        let handle = tokio::spawn(async move {
            Server::builder()
                .add_service(KvStoreServer::new(service))
                .serve(addr.parse().unwrap())
                .await
                .unwrap();
        });
    
        handles.push(handle);
    }    

    // router and load balancer
    let router = Router::new(server_addrs.clone());
    let lb = LoadBalancer::new(server_addrs);

    // test
    let key = "example-key";
    let client = router.route_request(key).await;
    let next_server = lb.get_next_server().await;
    println!("Routing key '{}' to next server: {}", key, next_server);

    // wait for servers to finish
    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
