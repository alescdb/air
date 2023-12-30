use std::fs;
use chrono::{Local, NaiveDateTime};
use openai::chat::{ChatCompletionMessage, ChatCompletionMessageRole};
use serde::{Deserialize, Serialize};
use crate::path::get_config_path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    date: NaiveDateTime,
    user: String,
    assistant: String,
}

pub struct History {
    file: String,
    exists: bool,
    expiration: u32,
    messages: Vec<Message>,
}

impl History {
    pub fn new(expiration: u32) -> Self {
        let config = get_config_path("history.json");
        return History {
            file: config.path,
            exists: config.exists,
            messages: vec![],
            expiration,
        };
    }
    pub fn add(&mut self, user: &str, assistant: &str) {
        let local_time = Local::now();
        let naive_time: NaiveDateTime = local_time.naive_local();

        self.messages.push(Message {
            date: naive_time,
            user: user.to_string(),
            assistant: assistant.to_string(),
        });
    }
    pub fn save(&self) {
        let serialized = serde_json::to_string_pretty(&self.messages)
            .expect("to_string_pretty() failed");
        fs::write(&self.file, serialized.as_str())
            .expect("read_to_string() failed");
    }
    pub fn get_completions(&self) -> Vec<ChatCompletionMessage> {
        let mut completions = vec![];
        for message in &self.messages {
            completions.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(message.user.to_owned()),
                name: None,
                function_call: None,
            });
            completions.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::Assistant,
                content: Some(message.assistant.to_owned()),
                name: None,
                function_call: None,
            });
        }
        return completions;
    }

    pub fn is_expired(&self, date: NaiveDateTime) -> bool {
        let now: NaiveDateTime = Local::now().naive_local();
        let duration = now.signed_duration_since(date).num_seconds();
        return duration > self.expiration as i64;
    }
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    pub fn load(&mut self) {
        if self.exists {
            let contents = fs::read_to_string(&self.file)
                .expect(&*format!("Failed to read file {}", self.file));
            let messages: Vec<Message> = serde_json::from_str(&contents)
                .expect(&*format!("Failed to parse json in {}", self.file));

            for message in messages {
                if !self.is_expired(message.date) {
                    self.messages.push(message);
                }
            }
        }
    }
}

