use tonic::transport::Channel;
use kvstore::kv_store_client::KvStoreClient;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod kvstore {
    tonic::include_proto!("kvstore");
}

fn calculate_server(key: &str, num_servers: usize) -> usize {
    let hash = seahash::hash(key.as_bytes());
    (hash as usize) % num_servers
}

pub struct Router {
    servers: Vec<String>,
}

impl Router {
    pub fn new(servers: Vec<String>) -> Self {
        Self { servers }
    }

    pub async fn route_request(&self, key: &str) -> KvStoreClient<Channel> {
        let server_idx = calculate_server(key, self.servers.len());
        let server_addr = &self.servers[server_idx];
        KvStoreClient::connect(format!("http://{}", server_addr))
            .await
            .expect("Failed to connect to server")
    }
}

pub struct LoadBalancer {
    servers: Vec<String>,
    index: Arc<Mutex<usize>>,
}

impl LoadBalancer {
    pub fn new(servers: Vec<String>) -> Self {
        Self {
            servers,
            index: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn get_next_server(&self) -> String {
        let mut idx = self.index.lock().await;
        let server = self.servers[*idx].clone();
        *idx = (*idx + 1) % self.servers.len();
        server
    }
}
