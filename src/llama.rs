use std::io::Write;

use crate::{
    ichat::{IChat, Message, Role},
    setup::LLamaSetup,
};
use async_trait::async_trait;
use libc::c_char;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

// https://github.com/mdrokz/rust-llama.cpp

pub struct LLamaChat {
    pub setup: LLamaSetup,
    pub system: Option<String>,
    pub verbose: bool,
}

#[async_trait]
impl IChat for LLamaChat {
    fn get_name(&mut self) -> &str {
        return "llama";
    }

    fn set_system(&mut self, system: String) {
        self.system = Some(system)
    }

    fn set_model(&mut self, _model: String) {
        // none
    }

    async fn chat(
        &mut self,
        prompt: String,
        history: Option<Vec<Message>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let model_options = ModelOptions {
            n_gpu_layers: self.setup.n_gpu_layers.unwrap_or(0),
            ..Default::default()
        };

        self.close_stderr();
        let llama = LLama::new(self.setup.model.clone(), &model_options).unwrap();
        self.open_stderr();

        let def = PredictOptions::default();
        let predict_options = PredictOptions {
            tokens: self.setup.tokens.unwrap_or(def.tokens),
            threads: self.setup.threads.unwrap_or(def.threads),
            top_k: self.setup.top_k.unwrap_or(def.top_k),
            top_p: self.setup.top_p.unwrap_or(def.top_p),
            temperature: self.setup.temperature.unwrap_or(def.temperature),
            token_callback: Some(Box::new(|token| {
                print!("{}", token);
                std::io::stdout().flush().unwrap();
                true
            })),
            ..Default::default()
        };

        let pfmt = self.get_prompt(&prompt, &history);

        log::info!("Temp.   : {}", predict_options.temperature);
        log::info!("Top_k   : {}", predict_options.top_k);
        log::info!("Top_p   : {}", predict_options.top_p);
        log::info!("Threads : {}", predict_options.threads);
        log::info!("Tokens  : {}", predict_options.tokens);
        log::info!("Prompt  : {}", pfmt);

        let answer = llama.predict(pfmt, predict_options)?;

        return Ok(answer);
    }
}

impl LLamaChat {
    fn get_prompt(&self, prompt: &str, history: &Option<Vec<Message>>) -> String {
        if self.setup.prompt.is_none() {
            return prompt.into();
        }

        let mut hst = "".to_string();
        if let Some(format) = &self.setup.history {
            if let Some(history) = history {
                let mut sh = format.clone();
                for item in history {
                    if item.role == Role::User {
                        sh = sh.replace("{user}", &item.content);
                    } else if item.role == Role::Assistant {
                        sh = sh.replace("{assistant}", &item.content);
                    }
                }
                // in case we missed one...
                hst += &sh
                    .replace("{user}", "")
                    .replace("{assistant}", "");
            }
        }

        return self
            .setup
            .prompt
            .clone()
            .unwrap()
            .replace("{system}", self.system.as_ref().unwrap_or(&"".into()))
            .replace("{prompt}", prompt)
            .replace("{history}", &hst);
    }

    #[allow(dead_code)]
    fn get_prompt_old(&self, prompt: &str) -> String {
        if self.setup.prompt.is_none() {
            return prompt.to_string();
        }
        let mut sys = "".to_string();
        if self.system.is_some() {
            sys = self.system.clone().unwrap();
        }
        return self
            .setup
            .prompt
            .clone()
            .unwrap()
            .replace("{system}", &sys)
            .replace("{prompt}", &prompt)
            .to_string();
    }

    pub fn new(setup: &LLamaSetup, verbose: bool) -> Self {
        return LLamaChat {
            setup: setup.clone(),
            system: None,
            verbose,
        };
    }

    fn close_stderr(&self) {
        if !self.verbose {
            unsafe {
                libc::close(libc::STDERR_FILENO);
            }
        }
    }

    fn open_stderr(&self) {
        if !self.verbose {
            unsafe {
                let wr = "w".as_ptr() as *const c_char;
                let fd = libc::fdopen(libc::STDERR_FILENO, wr);
                libc::dup2(fd as i32, libc::STDERR_FILENO);
            }
        }
    }
}
