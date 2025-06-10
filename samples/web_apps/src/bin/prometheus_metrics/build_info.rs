use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildInfo {
    pub build_time: chrono::DateTime<Utc>,
    pub commit_hash: String,
    pub package_version: String,
    pub rustc_version: String,
    pub manifest_path: String,
}

impl Default for BuildInfo {
    fn default() -> BuildInfo {
        let build_time = env!("BUILD_TIME");
        let commit_hash = env!("COMMIT_HASH");
        let package_version = env!("CARGO_PKG_VERSION");
        let rustc_version = env!("RUSTC_VERSION");
        let package_path = env!("CARGO_MANIFEST_PATH");
        BuildInfo {
            build_time: chrono::DateTime::parse_from_rfc3339(build_time)
                .unwrap_or_default()
                .to_utc(),
            commit_hash: commit_hash.to_string(),
            package_version: package_version.to_string(),
            rustc_version: rustc_version.to_string(),
            manifest_path: package_path.to_string(),
        }
    }
}
