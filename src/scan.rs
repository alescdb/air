use crate::setup::{LLamaSetup, Setup};
use std::fs;

pub fn scan_folder(setup: &Setup, path: &str) {
    let mut list = Vec::<LLamaSetup>::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("gguf") {
                        if !exists(setup, path.to_str().unwrap()) {
                            if let Some(base) = path.file_stem().and_then(|s| s.to_str()) {
                                list.push(LLamaSetup {
                                    name: get_name(base).to_lowercase().to_string(),
                                    model: path.to_str().unwrap().to_string(),
                                    prompt: None,
                                    history: None,
                                    temperature: None,
                                    n_gpu_layers: None,
                                    tokens: None,
                                    threads: None,
                                    top_k: None,
                                    top_p: None,
                                });
                            }
                        } else {
                            println!("Model already exists: {}", path.to_str().unwrap());
                        }
                    }
                }
            }
            println!("{}", serde_json::to_string_pretty(&list).unwrap());
        }
        Err(e) => {
            println!("Erreur lors de la lecture du répertoire : {}", e);
        }
    }
}

fn exists(setup: &Setup, file: &str) -> bool {
    if let Some(local) = &setup.local {
        for i in local {
            if i.model == file {
                return true;
            }
        }
    }
    return false;
}

fn get_name(base: &str) -> &str {
    if let Some(index) = base.find('-') {
        return &base[..index];
    }
    return base;
}
