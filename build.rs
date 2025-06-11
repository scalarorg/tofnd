use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: client build is needed only for tests https://github.com/rust-lang/cargo/issues/1581

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        // .build_client(false)
        // .out_dir(".") // if you want to peek at the generated code
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .compile_protos(&["proto/multisig.proto", "proto/common.proto"], &["proto"])?;
    Ok(())
}
