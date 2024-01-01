use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[allow(dead_code)]
impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::User => "user",
            Role::System => "system",
            Role::Assistant => "assistant", // Notez que les deux derniers ont la même valeur de chaîne, tout comme dans l'exemple C#.
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[async_trait]
pub trait IChat {
    fn get_name(&mut self) -> &str;
    fn set_system(&mut self, system: String);
    fn set_model(&mut self, model: String);
    async fn chat(&mut self, prompt: String, history: Option<Vec<Message>>) -> String;
}
