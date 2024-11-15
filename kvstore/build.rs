fn main() {
    tonic_build::compile_protos("kvstore.proto").expect("Failed to compile kvstore.proto");
}