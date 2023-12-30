use openai::chat::ChatCompletionMessage;
use async_trait::async_trait;

#[async_trait]
pub trait IChat {
    fn get_name(&mut self) -> &str;
    fn set_system(&mut self, system: String);
    fn set_model(&mut self, model: String);
    async fn chat(&mut self, prompt: String, history: Option<Vec<ChatCompletionMessage>>) -> String;
}