use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("instantchat_descriptor.bin"))
        .out_dir(&out_dir)
        .compile_protos(&["proto/instantchat/v1/instantchat.proto"], &["proto"])
        .unwrap();
}
