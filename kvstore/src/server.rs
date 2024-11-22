use tonic::{Request, Response, Status};
use kvstore::kv_store_server::{KvStore, KvStoreServer};
use kvstore::{PutRequest, PutResponse, GetRequest, GetResponse, DeleteRequest, DeleteResponse};
use rust_rocksdb::{DB, Options};

use seahash;

pub mod kvstore {
    tonic::include_proto!("kvstore");
}

// -------------------------------- Sharding --------------------------------

fn calculate_shard(key: &str, num_shards: usize) -> usize {
    let hash = seahash::hash(key.as_bytes()); // best hash function
    (hash as usize) % num_shards
}

fn init_shards(num_shards: usize, instance_id: u16) -> Vec<DB> {
    let mut shards = Vec::new();
    for i in 0..num_shards {
        let path = format!("shard_{}_{}", instance_id, i); // instance_id for uniqueness
        let mut options = Options::default();
        options.create_if_missing(true);
        let db = DB::open(&options, &path).unwrap();
        shards.push(db);
    }
    shards
}

// -------------------------------- Sharding --------------------------------

pub struct KVStoreService {
    shards: Vec<DB>,
}

impl KVStoreService {
    pub fn new(num_shards: usize, instance_id: u16) -> Self {
        let shards = init_shards(num_shards, instance_id);
        Self { shards }
    }

    fn get_shard(&self, key: &str) -> &DB {
        let shard_index = calculate_shard(key, self.shards.len());
        &self.shards[shard_index]
    }
}

#[tonic::async_trait]
impl KvStore for KVStoreService {
    async fn put(&self, request: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        let req = request.into_inner();
        let db = self.get_shard(&req.key); // get appropriate shard
        db.put(req.key.as_bytes(), req.value.as_bytes()).unwrap();
        Ok(Response::new(PutResponse { success: true }))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = request.into_inner();
        let db = self.get_shard(&req.key); // get appropriate shard
        if let Ok(Some(value)) = db.get(req.key.as_bytes()) {
            let value = String::from_utf8(value).unwrap();
            Ok(Response::new(GetResponse { value, found: true }))
        } else {
            Ok(Response::new(GetResponse { value: "".into(), found: false }))
        }
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let req = request.into_inner();
        let db = self.get_shard(&req.key); // get appropriate shard
        db.delete(req.key.as_bytes()).unwrap();
        Ok(Response::new(DeleteResponse { success: true }))
    }
}
