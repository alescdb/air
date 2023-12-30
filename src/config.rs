use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use crate::path::{FileInfo, get_config_path};

const EMPTY_KEY: &str = "<enter your openai api key here>";
#[derive(Serialize, Deserialize, Debug)]
pub struct Setup {
    pub apikey: String,
    pub model: String,
    pub system: String,
    pub markdown: bool,
    pub expiration: u32,
}

pub fn load_setup() -> Result<Setup, std::io::Error> {
    let config = get_config_path("setup.json");
    if !config.exists {
        write_setup(&config);
        return Err(std::io::Error::new(
            ErrorKind::NotFound,
            format!("Setup file {} does not exists !", &config.path))
        );
    }

    let contents = fs::read_to_string(config.path.clone())?;
    let setup: Setup = serde_json::from_str(&contents)?;

    if setup.apikey.is_empty() || setup.apikey == EMPTY_KEY {
        return Err(std::io::Error::new(
            ErrorKind::NotFound,
            format!("Edit setup file {}, and set your api key !", &config.path))
        );
    }
    return Ok(setup);
}

fn write_setup(config: &FileInfo) {
    let serialized = serde_json::to_string_pretty(&Setup {
        apikey: EMPTY_KEY.to_string(),
        model: "gpt-4".to_string(),
        system: "Your are a Linux assistant and a coder.".to_string(),
        markdown: true,
        expiration: 600,
    }).expect("to_string_pretty() failed");

    fs::write(&config.path, serialized.as_str())
        .expect("read_to_string() failed");
}

pub fn display_setup(setup: &Setup) {
    termimad::print_inline(&format!("*APIKEY*     => `{}`\n", setup.apikey));
    termimad::print_inline(&format!("*MODEL*      => `{}`\n", setup.model));
    termimad::print_inline(&format!("*SYSTEM*     => `{}`\n", setup.system));
    termimad::print_inline(&format!("*MARKDOWN*   => `{}`\n", setup.markdown));
    termimad::print_inline(&format!("*EXPIRATION* => `{}`\n", setup.expiration));
    termimad::print_inline("___\n");
}
