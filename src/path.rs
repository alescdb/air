use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct FileInfo {
    pub(crate) path: String,
    pub(crate) exists: bool,
}

pub fn get_config_directory() -> String {
    let config = dirs::config_dir()
        .unwrap()
        .join("air");

    let path = Path::new(&config);
    if !path.exists() || !path.is_dir() {
        fs::create_dir_all(&path).unwrap();
    }

    return config.to_string_lossy().to_string();
}

#[allow(dead_code)]
pub fn get_config_path(file: &str) -> FileInfo {
    let file = Path::new(&get_config_directory()).join(file);
    return FileInfo {
        path: file.to_string_lossy().to_string(),
        exists: file.exists() && file.is_file(),
    };
}