use chrono::Utc;
use std::process::Command;

fn main() {
    // Get current time in RFC3339 format
    let build_time = Utc::now().to_rfc3339();

    // Get the current commit hash using git
    let commit_hash = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Failed to get git commit hash")
        .stdout;
    let commit_hash = String::from_utf8(commit_hash)
        .expect("Invalid UTF-8 in commit hash")
        .trim()
        .to_string();

    // Get rustc version using rustc --version
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to get rustc version")
        .stdout;
    let rustc_version = String::from_utf8(rustc_version)
        .expect("Invalid UTF-8 in rustc version")
        .trim()
        .to_string();

    // Pass the information to the Rust binary as environment variables
    println!("cargo::rustc-env=BUILD_TIME={}", build_time);
    println!("cargo::rustc-env=COMMIT_HASH={}", commit_hash);
    println!("cargo::rustc-env=RUSTC_VERSION={}", rustc_version);

    // Instruct Cargo to rerun this build script if any of the relevant files change
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/");
}
