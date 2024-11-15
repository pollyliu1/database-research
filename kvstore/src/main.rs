use rocksdb::{DB, Options};
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::thread;

const NUM_OPS: usize = 1_000_000; // Number of operations
const NUM_THREADS: usize = 4; // Number of threads for multi-threading

fn init_db(path: &str) -> DB {
    let mut options = Options::default();
    options.create_if_missing(true);
    DB::open(&options, path).expect("Failed to open RocksDB")
}

fn put(db: &DB, key: &str, value: &str) {
    db.put(key.as_bytes(), value.as_bytes()).expect("Failed to write to RocksDB");
}

fn get(db: &DB, key: &str) -> Option<String> {
    match db.get(key.as_bytes()) {
        Ok(Some(value)) => Some(String::from_utf8(value).expect("Failed to read value")),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Error reading from RocksDB: {:?}", e);
            None
        }
    }
}

fn delete(db: &DB, key: &str) {
    db.delete(key.as_bytes()).expect("Failed to delete from RocksDB");
}

fn benchmark_put(db: Arc<DB>, num_ops: usize) {
    let start = Instant::now();
    for i in 0..num_ops {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        put(&db, &key, &value);
    }
    let duration = start.elapsed();
    let ops_per_sec = num_ops as f64 / duration.as_secs_f64();
    println!("PUT - Total Time: {:?}, Throughput: {:.2} ops/sec", duration, ops_per_sec);
}

fn benchmark_get(db: Arc<DB>, num_ops: usize) {
    let start = Instant::now();
    let mut found = 0;
    for i in 0..num_ops {
        let key = format!("key{}", i);
        if get(&db, &key).is_some() {
            found += 1;
        }
    }
    let duration = start.elapsed();
    let ops_per_sec = num_ops as f64 / duration.as_secs_f64();
    println!("GET - Total Time: {:?}, Throughput: {:.2} ops/sec, Found: {}", duration, ops_per_sec, found);
}

fn benchmark_delete(db: Arc<DB>, num_ops: usize) {
    let start = Instant::now();
    for i in 0..num_ops {
        let key = format!("key{}", i);
        delete(&db, &key);
    }
    let duration = start.elapsed();
    let ops_per_sec = num_ops as f64 / duration.as_secs_f64();
    println!("DELETE - Total Time: {:?}, Throughput: {:.2} ops/sec", duration, ops_per_sec);
}

fn benchmark_put_multithreaded(db: Arc<DB>, num_ops: usize, num_threads: usize) {
    let ops_per_thread = num_ops / num_threads;
    let start = Instant::now();

    let handles: Vec<_> = (0..num_threads).map(|t| {
        let db_clone = Arc::clone(&db);
        thread::spawn(move || {
            for i in 0..ops_per_thread {
                let idx = t * ops_per_thread + i;
                let key = format!("key{}", idx);
                let value = format!("value{}", idx);
                put(&db_clone, &key, &value);
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let ops_per_sec = num_ops as f64 / duration.as_secs_f64();
    println!("PUT (Multi-threaded) - Total Time: {:?}, Throughput: {:.2} ops/sec", duration, ops_per_sec);
}

fn benchmark_put_latency(db: Arc<DB>, num_ops: usize) {
    let mut total_duration = Duration::new(0, 0);
    for i in 0..num_ops {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        let start = Instant::now();
        put(&db, &key, &value);
        total_duration += start.elapsed();
    }
    let avg_latency = total_duration / num_ops as u32;
    println!("PUT - Average Latency: {:?}", avg_latency);
}

fn main() {
    let db_path = "kvstore_db";
    let db = init_db(db_path);
    let db = Arc::new(db); // Wrap in Arc for thread-safe sharing

    // // PUT
    // put(&db, "key1", "value2");
    // println!("Inserted key1 with value2");
    //
    // // GET
    // match get(&db, "key1") {
    //     Some(value) => println!("Retrieved key1: {}", value),
    //     None => println!("Key not found"),
    // }
    //
    // // DELETE
    // delete(&db, "key1");
    // println!("Deleted key1");
    // Single-threaded benchmarking
    benchmark_put(Arc::clone(&db), NUM_OPS);
    benchmark_get(Arc::clone(&db), NUM_OPS);
    benchmark_delete(Arc::clone(&db), NUM_OPS);

    // Multi-threaded benchmarking
    benchmark_put_multithreaded(Arc::clone(&db), NUM_OPS, NUM_THREADS);

    // Latency measurement
    benchmark_put_latency(Arc::clone(&db), NUM_OPS);
}
