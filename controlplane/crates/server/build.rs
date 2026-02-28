fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let descriptor_path = std::path::PathBuf::from(&out_dir).join("descriptor.bin");

    tonic_prost_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(
            &["../../../protocol/france_nuage/resourcemanager/v1/resourcemanager.proto"],
            &["../../../protocol"],
        )?;

    Ok(())
}
