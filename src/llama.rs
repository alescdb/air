use async_trait::async_trait;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use openai::chat::ChatCompletionMessage;
use crate::ichat::IChat;

pub struct LLamaChat {
    pub model: String,
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

    async fn chat(&mut self, prompt: String, _history: Option<Vec<ChatCompletionMessage>>) -> String {
        let model_options: ModelOptions = ModelOptions::default();

        let llama = LLama::new(
            self.model.clone(),
            &model_options,
        ).unwrap();

        let predict_options = PredictOptions {
            debug_mode: false,
            temperature: 0.2,
            tokens: 512,
            ..Default::default()
        };

        let results = llama.predict(
            prompt.into(),
            predict_options,
        ).unwrap();

        return results;
    }
}

impl LLamaChat {
    pub fn new(model: String) -> Self {
        return LLamaChat {
            model,
            system: None,
        };
    }
}