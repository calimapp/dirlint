use std::path::Path;

use clap::Error;

use crate::config::{Config, Folder};


pub fn lint_directory(path: &Path, config: &Config) -> Result<(), ()> {
    let mut errors: Vec<String> = Vec::new();
    check_folder(path, &config.structure, &mut errors);
    if errors.is_empty() {
        println!("✅ Lint passed: all expected files and folders are present.");
        Ok(())
    } else {
        for error in &errors {
            eprintln!("❌ {}", error);
        }
        Err(())
    }
}

fn check_folder(path: &Path, folder: &Folder, errors: &mut Vec<String>) {
    if !path.exists() {
        errors.push(format!("Missing directory: {}", path.display()));
    }
    // Check files
    for file in &folder.files {
        let file_path = path.join(file);
        if !file_path.exists() {
            errors.push(format!("Missing file: {}", file_path.display()));
        }
    }
    // Recurse into subfolders
    for (name, subfolder) in &folder.subfolders {
        let sub_path = path.join(name);
        check_folder(&sub_path, &subfolder, errors);
    }
}