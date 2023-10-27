use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]\n#[serde(rename_all=\"camelCase\")]")
        .emit_rerun_if_changed(true)
        .compile(&["Ecdar-ProtoBuf/services.proto"], &["Ecdar-ProtoBuf"])?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
