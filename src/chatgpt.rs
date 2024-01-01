use crate::ichat::IChat;
use async_trait::async_trait;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};

pub struct ChatGPT {
    pub apikey: String,
    pub model: String,
    pub system: Option<String>,
}

#[async_trait]
impl IChat for ChatGPT {
    fn get_name(&mut self) -> &str {
        return "chatgpt";
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
        history: Option<Vec<ChatCompletionMessage>>,
    ) -> String {
        let mut messages = vec![];

        if let Some(sys) = &self.system {
            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(sys.to_string()),
                name: None,
                function_call: None,
            });
        }
        if let Some(hs) = history {
            for h in hs {
                messages.push(h);
            }
        }
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(prompt.to_string()),
            name: None,
            function_call: None,
        });

        log::debug!("{:?}", messages);
        let completion = ChatCompletion::builder(&self.model, messages.clone())
            .create()
            .await
            .unwrap();
        let answer = completion.choices.first().unwrap().message.clone();
        return answer.content.clone().unwrap();
    }
}

impl ChatGPT {
    pub fn new(apikey: String) -> Self {
        set_key(apikey.clone());
        return ChatGPT {
            apikey,
            model: crate::setup::DEFAULT_MODEL.to_string(),
            system: None,
        };
    }
}
