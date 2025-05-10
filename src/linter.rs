use std::path::Path;


use crate::config::{Config, Folder};


pub fn lint_directory(path: &Path, config: &Config, strict: bool) -> Result<(), ()> {
    let mut errors: Vec<String> = Vec::new();
    check_folder(path, &config.structure, strict, &mut errors);
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

fn check_folder(path: &Path, folder: &Folder, strict: bool, errors: &mut Vec<String>) {
    if !path.exists() {
        errors.push(format!("Missing directory: {}", path.display()));
    }

    let Ok(entries) = std::fs::read_dir(path) else {
        errors.push(format!("Cannot read directory: {}", path.display()));
        return;
    };

    let expected_files: std::collections::HashSet<_> = folder.files.iter().map(|f| f.as_str()).collect();
    let expected_dirs: std::collections::HashSet<_> = folder.subfolders.keys().map(|k| k.as_str()).collect();

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if entry.path().is_dir() {
            if !expected_dirs.contains(name.as_ref()) {
                if strict {
                    errors.push(format!("Unexpected directory: {}", entry.path().display()));
                }
            }
        } else {
            if !expected_files.contains(name.as_ref()) {
                if strict {
                    errors.push(format!("Unexpected file: {}", entry.path().display()));
                }
            }
        }
    }
    // Recurse into subfolders
    for (name, subfolder) in &folder.subfolders {
        let sub_path = path.join(name);
        check_folder(&sub_path, &subfolder, strict, errors);
    }

    for expected_file in &folder.files {
        let file_path = path.join(expected_file);
        if !file_path.exists() {
            errors.push(format!("Missing file: {}", file_path.display()));
        }
    }
}