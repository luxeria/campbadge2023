use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg_toml = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../")
        .canonicalize()?
        .join("cfg.toml");

    println!("cargo:rerun-if-changed={}", cfg_toml.display());
    if !cfg_toml.exists() {
        println!("cargo:warning=Missing config file {:?}", cfg_toml);
    }

    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;
    Ok(())
}
