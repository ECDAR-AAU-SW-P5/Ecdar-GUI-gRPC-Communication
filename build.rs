fn main() {
    tonic_build::compile_protos("Ecdar-ProtoBuf/services.proto").unwrap();
    println!("cargo:rerun-if-changed=Ecdar-ProtoBuf");
}
