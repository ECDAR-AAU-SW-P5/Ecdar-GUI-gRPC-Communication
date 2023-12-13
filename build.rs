use std::error::Error;

const ECDAR_PROTOBUFF_DIR : &str = "ECDAR_PROTOBUFF_DIR";
const ECDAR_PROTOBUFF_ROOT: &str = "ECDAR_PROTOBUFF_ROOT";

fn main() -> Result<(), Box<dyn Error>> {
    let mut dir = std::env::var(ECDAR_PROTOBUFF_DIR).unwrap_or("./Ecdar-ProtoBuf/".into());
    if dir.chars().last().unwrap() != '/' { dir += "/" };
    let root = std::env::var(ECDAR_PROTOBUFF_ROOT).unwrap_or("services.proto".into());

    println!("Compiling protobuffers in {dir}{root} for rust");

    let build = tonic_build::configure();

    #[cfg(not(feature = "server"))]
    let build = build.build_server(false);

    #[cfg(not(feature = "client"))]
    let build = build.build_client(false);

    #[cfg(feature = "serde")]
    let build = build.type_attribute(".", "#[ecdar_protobuff_rs_macros::serde_derive]");

    build
        .emit_rerun_if_changed(true)
        .compile(&[format!("{dir}{root}")], &[dir])?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={ECDAR_PROTOBUFF_DIR}");
    println!("cargo:rerun-if-env-changed={ECDAR_PROTOBUFF_ROOT}");

    Ok(())
}

