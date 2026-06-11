use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = PathBuf::from(r"C:\Projects\All\grpc-crosslang-communication-demo\proto\user.proto");
    let proto_include = PathBuf::from(r"C:\Projects\All\grpc-crosslang-communication-demo\proto\");

    tonic_prost_build::configure()
        .out_dir("src/pb/")
        .compile_protos(&[proto_file], &[proto_include])?;

    Ok(())
}