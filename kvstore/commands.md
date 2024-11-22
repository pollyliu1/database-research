# Build and run
cargo build
cargo run

# Rebuild and run
cargo clean
cargo build
cargo run

# Run server and client
cargo run --bin server
cargo run --bin client

# Run gRPC
cargo run --bin kvstore -- --num-servers 3 --base-port 50051

# Set environment variables for compilation (must run every session)
export ROCKSDB_LIB_DIR=$(brew --prefix rocksdb)/lib
export ROCKSDB_INCLUDE_DIR=$(brew --prefix rocksdb)/include
export ROCKSDB_STATIC=1

echo 'export PATH="/opt/homebrew/opt/llvm/bin:$PATH"' >> /Users/pollyliu/.zshrc
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"

# Docker
docker build -t rust-kvstore .
docker run --rm -it rust-kvstore

For Docker compose:
docker-compose up --build
