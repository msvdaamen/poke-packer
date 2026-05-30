use std::{env, path::PathBuf};

const PROTOS: &[&str] = &["hello.proto", "api/user.proto"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contracts_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("../../../../contracts");

    let protos = PROTOS
        .iter()
        .map(|proto| contracts_dir.join(proto))
        .collect::<Vec<_>>();

    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    let includes = vec![contracts_dir.clone()];
    let descriptor_path = PathBuf::from(env::var("OUT_DIR")?).join("api_descriptor.bin");

    tonic_prost_build::configure()
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&protos, &includes)?;

    Ok(())
}
