use crate::path::{get_config_path, FileInfo};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::io::ErrorKind;

pub(crate) const DEFAULT_MODEL: &str = "gpt-4-1106-preview";
const EMPTY_KEY: &str = "<enter your openai api key here>";
const DEFAULT_SYSTEM: &str = "Your are a Linux assistant and a coder.";
const DEFAULT_EXPIRATION: u32 = 60 * 60 * 24; // 24h

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLamaSetup {
    pub name: String,
    pub model: String,
    pub prompt: Option<String>,
    pub temperature: Option<f32>,
    pub n_gpu_layers: Option<i32>,
    pub tokens: Option<i32>,
    pub threads: Option<i32>,
    pub top_k: Option<i32>,
    pub top_p: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Setup {
    pub apikey: String,
    pub model: Option<String>,
    pub system: Option<String>,
    pub markdown: Option<bool>,
    pub expiration: Option<u32>,
    pub local: Option<Vec<LLamaSetup>>,
}

impl Default for Setup {
    fn default() -> Setup {
        Self {
            apikey: EMPTY_KEY.to_string(),
            model: Some(DEFAULT_MODEL.to_string()),
            system: Some(DEFAULT_SYSTEM.to_string()),
            markdown: Some(true),
            expiration: Some(DEFAULT_EXPIRATION),
            local: None,
        }
    }
}

#[allow(dead_code)]
impl Setup {
    pub fn new() -> Result<Self, std::io::Error> {
        let config = get_config_path("setup.json");
        if !config.exists {
            match Setup::write(&config) {
                Ok(_) => (),
                Err(e) => log::error!("{}", e),
            };
            return Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!("Setup file {} does not exists !", &config.path),
            ));
        }

        let contents: String = fs::read_to_string(config.path.clone())?;
        let setup: Setup = serde_json::from_str(&contents)?;

        if setup.apikey.is_empty() || setup.apikey == EMPTY_KEY {
            return Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!("Edit setup file {}, and set your api key !", &config.path),
            ));
        }
        return Ok(setup);
    }

    pub fn get_markdown(&self) -> bool {
        return self.markdown.unwrap_or(true);
    }

    pub fn get_expiration(&self) -> u32 {
        return self.expiration.unwrap_or(DEFAULT_EXPIRATION);
    }

    pub fn get_model(&self) -> String {
        return self.model.clone().unwrap_or(DEFAULT_MODEL.to_string());
    }

    pub fn get_system(&self) -> String {
        return self.system.clone().unwrap_or(DEFAULT_SYSTEM.to_string());
    }

    fn write(config: &FileInfo) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(&Setup {
            local: Some(vec![LLamaSetup {
                name: "llama2".to_string(),
                model: "/opt/models/llama.gguf".to_string(),
                prompt: None,
                temperature: Some(0.2),
                n_gpu_layers: None,
                tokens: None,
                threads: None,
                top_k: None,
                top_p: None,
            }]),
            ..Default::default()
        })?;

        fs::write(&config.path, serialized.as_str())?;
        Ok(())
    }

    pub fn display(&self) {
        termimad::print_inline(&format!("*APIKEY*     => `{}`\n", self.apikey));
        termimad::print_inline(&format!("*MODEL*      => `{}`\n", self.get_model()));
        termimad::print_inline(&format!("*SYSTEM*     => `{}`\n", self.get_system()));
        termimad::print_inline(&format!("*MARKDOWN*   => `{}`\n", self.get_markdown()));
        termimad::print_inline(&format!("*EXPIRATION* => `{}`\n", self.get_expiration()));

        if let Some(local) = &self.local {
            for (i, llama) in local.iter().enumerate() {
                termimad::print_inline(&format!("# LOCAL {}\n", i));
                termimad::print_inline(&format!("- *NAME*     => `{:?}`\n", llama.name));
                termimad::print_inline(&format!("- *MODEL*    => `{:?}`\n", llama.model));
                termimad::print_inline(&format!("- *PROMPT*   => `{:?}`\n", llama.prompt));
            }
        }
        termimad::print_inline("___\n");
    }
}
