// src/binary/mod.rs
pub mod binary_analysis;
pub mod secret_scanner;
pub mod metadata_extractor;
pub mod generate_sbom;

pub use self::binary_analysis::analyze_binary;
pub use self::secret_scanner::{SecretScanner, SecretScanResult};
pub use self::metadata_extractor::{extract_version_info, extract_license_info, VersionInfo, LicenseInfo};
pub use self::generate_sbom::{generate_sbom, generate_package_sbom};

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinaryAnalysis {
    pub id: Uuid,
    pub file_name: String,
    pub format: String,
    pub architecture: String,
    pub languages: Vec<String>,
    pub detected_symbols: Vec<String>,
    pub embedded_strings: Vec<String>,
    pub suspected_secrets: Vec<String>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub hash_sha256: String,
    pub hash_blake3: Option<String>,
    pub size_bytes: u64,
    pub linked_libraries: Vec<String>,
    pub static_linked: bool,
    pub version_info: Option<VersionInfo>,
    pub license_info: Option<LicenseInfo>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub sbom: Option<serde_json::Value>,
}
