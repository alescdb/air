use crate::setup::{LLamaSetup, Setup};
use regex::Regex;
use std::fs;

pub fn scan_folder(setup: &mut Setup, path: &str) {
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
                                    name: get_name(base),
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
            if !list.is_empty() {
                println!("{}", serde_json::to_string_pretty(&list).unwrap());
                setup.add_locals(list);
                setup.save();
            } else {
                println!("No (new) models found");
            }
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

fn get_name(base: &str) -> String {
    let name = base.to_lowercase();
    let params = Regex::new(r"-[0-9]+b[$|\.|-]").unwrap();
    let expr = if params.is_match(&name) {
        r"^(gguf-|ggml-)?([a-z|0-9|-]+)(-[0-9]+b)"
    } else {
        r"^(gguf-|ggml-)?([a-z|0-9|-]+)"
    };
    let re = Regex::new(expr).unwrap();

    match re.captures(&name) {
        Some(caps) => {
            log::debug!("REGEX   : {}", name);
            log::debug!("Group 1 : {}", caps.get(1).map_or("", |m| m.as_str()));
            log::debug!("Group 3 : {}", caps.get(3).map_or("", |m| m.as_str()));
            log::debug!("---");
            log::debug!("Final   : {}", caps.get(2).map_or("", |m| m.as_str()));
            log::debug!("\n");
            if let Some(fname) = caps.get(2) {
                return fname.as_str().to_string();
            }
            return name;
        }
        None => {
            return name;
        }
    }
}
