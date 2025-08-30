use std::{fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from("./src/generated");
    let proto_file = "proto/v1/instant_chat.proto";

    // Clean up before re-generating
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir).expect("failed to remove output directory");
    }

    fs::create_dir_all(&out_dir).expect("failed to create output directory");
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("instant_chat_descriptor.bin"))
        .out_dir(&out_dir)
        .compile_protos(&[proto_file], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {e}"));
    println!("cargo:rerun-if-changed={proto_file}");
}
