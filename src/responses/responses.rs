use std::{env, error::Error};
use std::fmt::{self, Display};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::responses::{request, input, common, tools, response};

#[derive(Debug, Serialize, Deserialize)]
pub struct Responses {
    api_key: String,
    api_url: String,
    request: request::Request,
}
impl Responses {
    pub fn new(api_key: String, model: String, api_url: Option<String>) -> Self {
        Self {
            api_key,
            api_url: api_url.unwrap_or("https://api.groq.com/openai/v1/responses".to_string()),
            request: request::Request::new(model, None),
        }
    }

    pub fn set_input(&mut self, input: Option<input::Input>) {
        self.request.input = input;
    }

    pub fn add_input_item(&mut self, input_item: input::InputItem) {
        if let Some(input) = &mut self.request.input {
            if let input::Input::InputItemList(items) = input {
                items.push(input_item);
            }
        }
    }

    pub fn set_system_prompt(&mut self, system_prompt: String) {
        self.request.instructions = Some(system_prompt);
    }

    pub fn set_instructions(&mut self, instructions: String) {
        self.request.instructions = Some(instructions);
    }

    pub fn set_max_output_tokens(&mut self, max_output_tokens: u64) {
        self.request.max_output_tokens = Some(max_output_tokens);
    }

    pub fn set_metadata(&mut self, metadata: HashMap<String, serde_json::Value>) {
        self.request.metadata = Some(metadata);
    }

    pub fn set_parallel_tool_calls(&mut self, parallel_tool_calls: bool) {
        self.request.parallel_tool_calls = Some(parallel_tool_calls);
    }

    pub fn set_service_tier(&mut self, service_tier: common::ServiceTier) {
        self.request.service_tier = Some(service_tier);
    }

    pub fn set_store(&mut self, store: bool) {
        self.request.store = Some(store);
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.request.temperature = Some(temperature);
    }

    pub fn set_tool_choice(&mut self, tool_choice: tools::ToolChoice) {
        self.request.tool_choice = Some(tool_choice);
    }

    pub fn set_tools(&mut self, tools: Vec<tools::Tool>) {
        self.request.tools = Some(tools);
    }

    pub fn add_tool(&mut self, tool: tools::Tool) {
        match &mut self.request.tools {
            Some(tools) => {
                tools.push(tool);
            }
            None => {
                self.request.tools = Some(vec![tool]);
            }
        }
    }

    pub fn set_top_p(&mut self, top_p: f64) {
        self.request.top_p = Some(top_p);
    }

    pub fn set_truncation(&mut self, truncation: common::Truncation) {
        self.request.truncation = Some(truncation);
    }

    pub fn set_model(&mut self, model: String) {
        self.request.model = model;
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }

    pub fn set_api_url(&mut self, api_url: String) {
        self.api_url = api_url;
    }

    pub async fn send(&self) -> Result<response::Response, Box<dyn Error + Send + Sync>> {
        let mut client_builder = reqwest::Client::builder();
        if let Ok(proxy) = env::var("HTTPS_PROXY"){
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy)?);
        }
        let client = client_builder.build()?;
        let body = serde_json::to_string(&self.request)?;
        let response = match client
            .post(self.api_url.clone())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(body)
            .send()
            .await{
                Ok(response) => response,
                Err(e) => {
                    return Err(Box::new(e));
                }
            };
        if response.status().is_client_error() {
            let raw_response = response.text().await?;
            eprintln!("{}", raw_response);
            let responses_error = serde_json::from_str::<ResponsesError>(&raw_response)?;
            return Err(Box::new(responses_error));
        }
        let raw_response = response.text().await?;
        let responses_response = serde_json::from_str::<response::Response>(&raw_response)?;
        Ok(responses_response)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Error)]
pub struct ResponsesError {
    error: ResponsesErrorDetails,
}

impl Display for ResponsesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Responses error: {}", self.error.message)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ResponsesErrorDetails {
    message: String,
    r#type: String,
    param: Option<String>,
    code: Option<String>,
}




