# Build and run
cargo build
cargo run

# Rebuild and run
cargo clean
cargo build
cargo run

# Set environment variables for compilation (must run every session)
export ROCKSDB_LIB_DIR=$(brew --prefix rocksdb)/lib
export ROCKSDB_INCLUDE_DIR=$(brew --prefix rocksdb)/include
export ROCKSDB_STATIC=1
