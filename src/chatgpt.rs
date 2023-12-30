use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};

pub struct ChatGPT {
    pub apikey: String,
    pub model: String,
    pub system: Option<String>,
}

impl ChatGPT {
    pub fn new(api_key: String) -> Self {
        set_key(api_key.clone());
        return ChatGPT {
            apikey: api_key,
            model: "gpt-4-1106-preview".to_string(),
            system: None,
        };
    }

    pub fn set_system(&mut self, system: String) {
        self.system = Some(system)
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model
    }

    pub async fn chat(&mut self, prompt: String, history: Option<Vec<ChatCompletionMessage>>) -> String {
        let mut messages = vec![];
        if self.system != None && self.system.is_some() {
            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: self.system.clone(),
                name: None,
                function_call: None,
            });
        }
        if !history.is_none() {
            for h in history.unwrap() {
                messages.push(h);
            }
        }
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(prompt.to_string()),
            name: None,
            function_call: None,
        });

        // println!("{:?}", messages);
        let completion = ChatCompletion::builder(&self.model, messages.clone())
            .create()
            .await
            .unwrap();
        let answer = completion.choices.first().unwrap().message.clone();
        return answer.content.clone().unwrap();
    }
}
