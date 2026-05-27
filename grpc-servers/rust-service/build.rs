fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = r"C:\Projects\All\grpc-crosslang-communication-demo\proto\user.proto";
    let fbs_file = r"C:\Projects\All\grpc-crosslang-communication-demo\fbs\user.fbs";

    // Re-run if schema files change
    println!("cargo:rerun-if-changed={}", proto_file);
    println!("cargo:rerun-if-changed={}", fbs_file);

    // Compile Protobuf using the new tonic-prost-build crate
    tonic_prost_build::configure()
        .compile_protos(
            &[proto_file], 
            &[r"C:\Projects\All\grpc-crosslang-communication-demo\proto"]
        )?;

    // Compile FlatBuffers
    flatbuffers_tonic_build::compile_flatbuffers_tonic(&[fbs_file])?;

    Ok(())
}