fn main() {
    tonic_build::compile_protos("proto/implant.proto").unwrap();
    tonic_build::compile_protos("proto/operator.proto").unwrap();
}
