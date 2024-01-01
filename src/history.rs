use crate::{
    ichat::{self, Role},
    path::get_config_path,
};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryMessage {
    date: NaiveDateTime,
    chat: String,
    user: String,
    assistant: String,
}

pub struct History {
    file: String,
    exists: bool,
    expiration: u32,
    messages: Vec<HistoryMessage>,
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
    pub fn add(&mut self, chat: &str, user: &str, assistant: &str) {
        let local_time = Local::now();
        let naive_time: NaiveDateTime = local_time.naive_local();

        self.messages.push(HistoryMessage {
            date: naive_time,
            chat: chat.to_string(),
            user: user.to_string(),
            assistant: assistant.to_string(),
        });
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(&self.messages)?;
        fs::write(&self.file, serialized.as_str())?;
        Ok(())
    }

    pub fn get_completions(&self) -> Vec<ichat::Message> {
        let mut completions = vec![];
        for message in &self.messages {
            completions.push(ichat::Message {
                role: Role::User,
                content: message.user.to_owned(),
            });
            completions.push(ichat::Message {
                role: Role::Assistant,
                content: message.assistant.to_owned(),
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
    pub fn load(&mut self) -> Result<(), std::io::Error> {
        if self.exists {
            let contents = fs::read_to_string(&self.file)?;
            let messages: Vec<HistoryMessage> = serde_json::from_str(&contents)?;

            for message in messages {
                if !self.is_expired(message.date) {
                    self.messages.push(message);
                }
            }
        }
        Ok(())
    }
}
