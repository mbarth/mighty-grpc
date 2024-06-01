/// Entry point for the build script.
///
/// This script is responsible for compiling the Protobuf definitions using `tonic-build`.
/// It processes the `mighty_inference.proto` file located in the `src/proto` directory
/// and outputs the corresponding Rust definitions. Additionally, it generates a binary
/// descriptor set file used for gRPC reflection, enabling clients to understand the services
/// the gRPC server exposes, including the methods and message types, without having the
/// proto file at compile time.
///
/// # Errors
///
/// Returns `Err` if there is a problem compiling the Protobuf definitions.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("src/proto/mighty_inference.bin")
        .compile(&["src/proto/mighty_inference.proto"], &["proto"])?;
    Ok(())
}

