use crate::clients::client::Client;
use crate::clients::client::LLMResponse;
use async_trait::async_trait;

pub struct OpenAiClient {
    pub api_key: String,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl Client for OpenAiClient {
    async fn get_answer(
        &self,
        _prompt: &str,
    ) -> Result<LLMResponse, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement OpenAI API call

        let answer = "yay".to_string();
        let reason = "because of X and Y".to_string();

        println!("Answer: {}", answer);
        println!("Reason: {}", reason);

        Ok(LLMResponse { answer, reason })
    }
}
