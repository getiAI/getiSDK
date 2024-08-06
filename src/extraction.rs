// src/extraction.rs

use std::fs;
use std::path::{Path, PathBuf};
use log::{info, warn};

pub fn extract_contracts(path: Option<&str>) -> Vec<PathBuf> {
    let contract_path = match path {
        Some(p) => PathBuf::from(p),
        None => std::env::current_dir().expect("Failed to get current directory"),
    };

    let mut contract_files = Vec::new();
    if contract_path.is_dir() {
        info!("Extracting contracts from directory: {:?}", contract_path);
        extract_from_directory(&contract_path, &mut contract_files);
    } else {
        warn!("Provided path is not a directory: {:?}", contract_path);
    }

    contract_files
}

fn extract_from_directory(dir: &Path, contract_files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to get directory entry");
        let path = entry.path();

        if path.is_dir() {
            info!("Entering directory: {:?}", path);
            extract_from_directory(&path, contract_files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            info!("Found contract file: {:?}", path);
            contract_files.push(path);
        } else {
            warn!("Skipping non-contract file: {:?}", path);
        }
    }
}
