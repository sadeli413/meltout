fn main() {
    tonic_build::compile_protos("../proto/implant.proto").unwrap()
}
