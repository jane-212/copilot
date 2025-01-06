use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestUserMessageContent, CreateChatCompletionRequestArgs,
};
use async_openai::Client;

pub struct Openai {
    client: Client<OpenAIConfig>,
    model: String,
}

impl Openai {
    pub fn new(
        base: impl Into<String>,
        model: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Self> {
        let config = OpenAIConfig::new().with_api_base(base).with_api_key(key);
        let client = Client::with_config(config);
        let openai = Self {
            client,
            model: model.into(),
        };

        Ok(openai)
    }

    pub async fn chat(
        &self,
        content: impl Into<ChatCompletionRequestUserMessageContent>,
    ) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.clone())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content("You are a helpful assistant.")
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()?
                    .into(),
            ])
            .build()?;
        let response = self.client.chat().create(request).await?;
        let reply = response
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .unwrap_or_default();

        Ok(reply)
    }
}
