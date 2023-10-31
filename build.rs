use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let build = tonic_build::configure();

    #[cfg(feature = "client")]
    let build = build.build_client(true);

    #[cfg(feature = "server")]
    let build = build.build_server(true);

    #[cfg(feature = "serde")]
    let build = build.type_attribute(
        ".",
        "#[derive(serde::Deserialize, serde::Serialize)]\n#[serde(rename_all=\"camelCase\")]",
    );

    build
        .emit_rerun_if_changed(true)
        .compile(&["Ecdar-ProtoBuf/services.proto"], &["Ecdar-ProtoBuf"])?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
