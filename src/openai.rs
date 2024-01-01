use crate::ichat::{IChat, Message, Role};
use async_trait::async_trait;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};

const HEADER_AUTHORIZATION: &str = "Authorization";
const HEADER_CONTENT_TYPE: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";
const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Debug)]
pub struct OpenAICompletion<'a> {
    model: &'a str,
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OpenAIChoice {
    index: u32,
    message: Message,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OpenAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u32>,
    model: Option<String>,
    choices: Vec<OpenAIChoice>,
    system_fingerprint: Option<String>,
}

pub struct OpenAI {
    pub apikey: String,
    pub model: String,
    pub system: Option<String>,
}

#[async_trait]
impl IChat for OpenAI {
    fn get_name(&mut self) -> &str {
        return "openai";
    }

    fn set_system(&mut self, system: String) {
        self.system = Some(system)
    }

    fn set_model(&mut self, model: String) {
        self.model = model
    }

    async fn chat(&mut self, prompt: String, history: Option<Vec<Message>>) -> String {
        let mut messages: Vec<Message> = vec![];

        if let Some(sys) = &self.system {
            messages.push(Message {
                role: Role::System,
                content: sys.to_string(),
            });
        }

        if let Some(hs) = history {
            for h in hs {
                messages.push(h);
            }
        }

        messages.push(Message {
            role: Role::User,
            content: prompt.to_string(),
        });

        let completion = OpenAICompletion {
            model: &self.model,
            messages,
        };
        let serialized =
            serde_json::to_string_pretty(&completion).expect("to_string_pretty() failed");
        log::debug!("{}\n", serialized);

        let client: Client = Client::new();
        let answer: OpenAIResponse = client
            .post(OPENAI_URL)
            .header(HEADER_AUTHORIZATION, format!("Bearer {}", self.apikey))
            .header(HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
            .body(serialized)
            .send()
            .await
            .expect("send() failed")
            .json::<OpenAIResponse>()
            .await
            .expect("text() failed");

        log::debug!("{:?}\n", answer);
        return answer.choices.first().unwrap().message.content.clone();
    }
}

impl OpenAI {
    pub fn new(apikey: String) -> Self {
        return OpenAI {
            apikey,
            model: crate::setup::DEFAULT_MODEL.to_string(),
            system: None,
        };
    }
}
