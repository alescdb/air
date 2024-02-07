use crate::{
    ichat::{IChat, Message, Role},
};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tokio_stream::StreamExt;

const HEADER_AUTHORIZATION: &str = "Authorization";
const HEADER_CONTENT_TYPE: &str = "Content-Type";
const CONTENT_TYPE_JSON: &str = "application/json";
const OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Serialize, Debug)]
pub struct OpenAICompletion<'a> {
    model: &'a str,
    messages: Vec<Message>,
    stream: bool,
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
pub struct OpenAIError {
    message: Option<String>,
    #[serde(rename = "type")]
    error_type: Option<String>,
    param: Option<String>,
    code: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct OpenAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u32>,
    model: Option<String>,
    choices: Option<Vec<OpenAIChoice>>,
    system_fingerprint: Option<String>,
    error: Option<OpenAIError>,
}

pub struct OpenAI {
    pub apikey: String,
    pub model: String,
    pub system: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct StreamDelta {
    content: Option<String>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct StreamChoice {
    index: Option<u32>,
    delta: Option<StreamDelta>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct StreamChunk {
    id: Option<String>,
    object: Option<String>,
    created: Option<u32>,
    model: Option<String>,
    system_fingerprint: Option<String>,
    choices: Option<Vec<StreamChoice>>,
}

fn parse_data(line: &str) -> Result<String, Box<dyn std::error::Error>> {
    // println!("LINE => '{}'", line);
    let mut message = String::new();
    if line.starts_with("data: ") && !line.starts_with("data: [DONE]") {
        let json: StreamChunk = serde_json::from_str(&line[6..])?;

        if let Some(choices) = json.choices {
            for c in choices {
                if let Some(delta) = c.delta {
                    if let Some(content) = delta.content {
                        print!("{}", content);
                        message.push_str(&content);
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
        }
    }
    Ok(message)
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

    async fn chat(
        &mut self,
        prompt: String,
        history: Option<Vec<Message>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut messages: Vec<Message> = vec![];

        if let Some(sys) = &self.system {
            if sys.len() > 0 {
                messages.push(Message {
                    role: Role::System,
                    content: sys.to_string(),
                });
            }
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
            stream: true,
            messages,
        };
        let serialized: String = serde_json::to_string_pretty(&completion)?;
        log::debug!("{}\n", serialized);

        let client: Client = Client::new();
        let response = client
            .post(OPENAI_URL)
            .header(HEADER_AUTHORIZATION, format!("Bearer {}", self.apikey))
            .header(HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
            .body(serialized)
            .send()
            .await?;

        let mut stream = response.bytes_stream();
        let mut message: String = String::new();
        let mut buffer: Vec<u8> = vec![];

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    for c in bytes {
                        if c == b'\n' {
                            if buffer.len() > 0 {
                                let line = parse_data(&String::from_utf8(buffer.clone())?)?;
                                message.push_str(&line);
                                message.push('\n');
                            }
                            buffer = vec![];
                        } else {
                            buffer.push(c);
                        }
                    }
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
        }
        Ok(message)
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
