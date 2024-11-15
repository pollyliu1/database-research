use tonic::{transport::Server, Request, Response, Status};
use kvstore::kv_store_server::{KvStore, KvStoreServer};
use kvstore::{PutRequest, PutResponse, GetRequest, GetResponse, DeleteRequest, DeleteResponse};
use rust_rocksdb::{DB, Options};

pub mod kvstore {
    tonic::include_proto!("kvstore");
}

pub struct KVStoreService {
    db: DB,
}

impl KVStoreService {
    pub fn new(path: &str) -> Self {
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = DB::open(&options, path).unwrap();
        Self { db }
    }
}

#[tonic::async_trait]
impl KvStore for KVStoreService {
    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        let req = request.into_inner();
        self.db.put(req.key.as_bytes(), req.value.as_bytes()).unwrap();
        Ok(Response::new(PutResponse { success: true }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        if let Ok(Some(value)) = self.db.get(req.key.as_bytes()) {
            let value = String::from_utf8(value).unwrap();
            Ok(Response::new(GetResponse { value, found: true }))
        } else {
            Ok(Response::new(GetResponse { value: "".into(), found: false }))
        }
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        self.db.delete(req.key.as_bytes()).unwrap();
        Ok(Response::new(DeleteResponse { success: true }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = KVStoreService::new("kvstore_db");

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(KvStoreServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
