use crate::ichat::IChat;
use async_trait::async_trait;
use llama_cpp_rs::{
    options::{ModelOptions, PredictOptions},
    LLama,
};
use openai::chat::ChatCompletionMessage;

pub struct LLamaChat {
    pub model: String,
    pub system: Option<String>,
    pub prompt_template: Option<String>,
    main_gpu: String,
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
        _history: Option<Vec<ChatCompletionMessage>>,
    ) -> String {
        let model_options: ModelOptions = ModelOptions {
            main_gpu: self.main_gpu.clone(),
            ..Default::default()
        };

        let llama: LLama = LLama::new(self.model.clone(), &model_options).unwrap();

        let predict_options: PredictOptions = PredictOptions {
            debug_mode: false,
            temperature: 0.2,
            tokens: 512,
            main_gpu: self.main_gpu.clone(),
            ..Default::default()
        };
        let prompt_fmt = self.get_prompt(prompt);
        log::debug!("Prompt Template : {}", prompt_fmt);

        return llama.predict(prompt_fmt.into(), predict_options).unwrap();
    }
}

impl LLamaChat {
    fn get_prompt(&self, prompt: String) -> String {
        if self.prompt_template.is_none() {
            return prompt;
        }

        let mut sys = "".to_string();
        if self.system.is_some() {
            sys = self.system.clone().unwrap();
        }
        return self
            .prompt_template
            .clone()
            .unwrap()
            .replace("{system}", &sys)
            .replace("{prompt}", &prompt)
            .to_string();
    }
    pub fn new(model: String, prompt_template: Option<String>, main_gpu: String) -> Self {
        return LLamaChat {
            model,
            system: None,
            prompt_template,
            main_gpu,
        };
    }
}
