use crate::path::{get_config_path, FileInfo};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

pub(crate) const DEFAULT_MODEL: &str = "gpt-4-1106-preview";
const EMPTY_KEY: &str = "<enter your openai api key here>";
const DEFAULT_SYSTEM: &str = "Your are a Linux assistant and a coder.";
const DEFAULT_EXPIRATION: u32 = 60 * 60 * 24; // 24h

const EX_NAME: &str = "vigogne";
const EX_MODEL: &str = "/opt/models/vigogne-2-7b-chat.Q4_K_M.gguf";
const EX_PROMPT: &str = "{system}\n\n{history}<|UTILISATEUR|>: {prompt}\n<|ASSISTANT|>: \n";
const EX_HISTORY: &str = "<|UTILISATEUR|>: {user}\n<|ASSISTANT|>: {assistant}\n";
const EX_N_GPU_LAYERS: i32 = 12;
const EX_TOKENS: i32 = 0;
const EX_THREADS: i32 = 14;
const EX_TOP_K: i32 = 90;
const EX_TOP_P: f32 = 0.8;
const EX_TEMPERATURE: f32 = 0.2;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LLamaSetup {
    pub name: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_gpu_layers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

impl LLamaSetup {
    pub fn model_exist(&self, llama: &LLamaSetup) -> bool {
        let path = Path::new(&llama.model);
        return path.exists();
    }
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
        let config: FileInfo = get_config_path("setup.json");
        if !config.exists {
            // match Setup::write(&config) {
            //     Ok(_) => (),
            //     Err(e) => log::error!("{}", e),
            // };
            return Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!(
                    "Setup file {} does not exists !\nSetup example:\n{}\n",
                    &config.path,
                    &Setup::get_example()?
                ),
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

    fn get_example() -> Result<String, serde_json::error::Error> {
        return Ok(serde_json::to_string_pretty(&Setup {
            local: Some(vec![LLamaSetup {
                name: EX_NAME.into(),
                model: EX_MODEL.into(),
                prompt: Some(EX_PROMPT.into()),
                history: Some(EX_HISTORY.into()),
                n_gpu_layers: Some(EX_N_GPU_LAYERS),
                tokens: Some(EX_TOKENS),
                threads: Some(EX_THREADS),
                top_k: Some(EX_TOP_K),
                top_p: Some(EX_TOP_P),
                temperature: Some(EX_TEMPERATURE),
            }]),
            ..Default::default()
        })?);
    }

    // #[allow(dead_code)]
    // fn write(config: &FileInfo, setup: &Setup) -> Result<(), std::io::Error> {
    //     fs::write(&config.path, setup.to_string())?;
    //     Ok(())
    // }

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

    pub(crate) fn add_locals(&mut self, list: Vec<LLamaSetup>) {
        if self.local.is_none() {
            self.local = Some(list);
        } else {
            self.local.as_mut().unwrap().extend(list);
        }
    }

    pub(crate) fn save(&self) {
        let config: FileInfo = get_config_path("setup.json");
        let _ = fs::write(&config.path, serde_json::to_string_pretty(&self).unwrap());
    }
}
