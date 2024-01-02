use std::io::Write;

use crate::{
    ichat::{IChat, Message},
    setup::LLamaSetup,
};
use async_trait::async_trait;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};

// https://github.com/mdrokz/rust-llama.cpp

pub struct LLamaChat {
    pub setup: LLamaSetup,
    pub system: Option<String>,
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
        _history: Option<Vec<Message>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let model_options = ModelOptions {
            n_gpu_layers: self.setup.n_gpu_layers.unwrap_or(0),
            ..Default::default()
        };
        let llama = LLama::new(self.setup.model.clone(), &model_options).unwrap();

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

        let pfmt = self.get_prompt(&prompt);

        log::debug!("Temp.   : {}", predict_options.temperature);
        log::debug!("Top_k   : {}", predict_options.top_k);
        log::debug!("Top_p   : {}", predict_options.top_p);
        log::debug!("Threads : {}", predict_options.threads);
        log::debug!("Tokens  : {}", predict_options.tokens);
        log::debug!("Prompt  : {}", pfmt);
        
        let _ = llama.predict(pfmt, predict_options)?;

        return Ok(String::from(""));
    }
}

impl LLamaChat {
    fn get_prompt(&self, prompt: &str) -> String {
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
    pub fn new(setup: &LLamaSetup) -> Self {
        return LLamaChat {
            setup: setup.clone(),
            system: None,
        };
    }
}
