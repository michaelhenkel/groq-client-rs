use std::{env, error::Error, fmt::{self, Display}, pin::Pin, task::{Context, Poll}};
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use serde_json::Value;
use std::io::{self, ErrorKind};
use tokio_util::io::StreamReader;
use tokio_util::codec::{FramedRead, LinesCodec};
use tokio::io::BufReader;
use serde::{de, Deserialize, Deserializer, Serialize};
use thiserror::Error;

pub struct BoxStreamUnpin<T>(Pin<Box<dyn Stream<Item = T> + Send>>);

impl<T> BoxStreamUnpin<T> {
    pub fn new<S>(stream: S) -> Self
    where
        S: Stream<Item = T> + Send + 'static,
    {
        BoxStreamUnpin(Box::pin(stream))
    }
}

impl<T> Stream for BoxStreamUnpin<T> {
    type Item = T;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().0.as_mut().poll_next(cx)
    }
}

impl<T> Unpin for BoxStreamUnpin<T> {}

pub trait BoxUnpinExt: Stream + Sized + Send + 'static {
    fn boxed_unpin(self) -> BoxStreamUnpin<Self::Item> {
        BoxStreamUnpin::new(self)
    }
}

impl<T: Stream + Sized + Send + 'static> BoxUnpinExt for T {}

#[derive(Clone)]
pub struct Chat {
    api_key: String,
    api_url: String,
    chat_request: ChatRequest,
}

impl Chat {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            api_url: "https://api.groq.com/openai/v1/chat/completions".to_string(),
            chat_request: ChatRequest::new(model, vec![]),
        }
    }

    pub fn set_chat_messages(&mut self, messages: Vec<ChatMessage>) {
        self.chat_request.messages = messages;
    }

    pub fn get_chat_messages(&self) -> Vec<ChatMessage> {
        self.chat_request.messages.clone()
    }

    pub fn set_api_url(&mut self, api_url: String) {
        self.api_url = api_url;
    }

    pub fn set_model(&mut self, model: String) {
        self.chat_request.model = model;
    }

    pub fn add_chat_message(&mut self, message: ChatMessage) {
        self.chat_request.messages.push(message);
    }

    pub fn clear_chat_messages(&mut self) {
        self.chat_request.messages.clear();
    }

    pub fn remove_last_n_chat_messages(&mut self, n: usize) {
        self.chat_request.messages.truncate(self.chat_request.messages.len() - n);
    }

    pub fn remove_first_n_chat_messages(&mut self, n: usize) {
        let to_remove = n.min(self.chat_request.messages.len());
        self.chat_request.messages.drain(0..to_remove);
    }

    pub fn remove_last_chat_message(&mut self) {
        self.chat_request.messages.pop();
    }

    pub fn number_of_chat_messages(&self) -> usize {
        self.chat_request.messages.len()
    }

    pub fn remove_first_chat_message(&mut self) {
        if !self.chat_request.messages.is_empty() {
            self.chat_request.messages.remove(0);
        }
    }

    pub fn set_frequency_penalty(&mut self, frequency_penalty: f32) -> Result<(), String> {
        if frequency_penalty < -2.0 || frequency_penalty > 2.0 {
            return Err("Frequency penalty must be between -2.0 and 2.0".to_string());
        }
        self.chat_request.frequency_penalty = frequency_penalty;
        Ok(())
    }

    pub fn set_max_completion_tokens(&mut self, max_completion_tokens: u32) {
        self.chat_request.max_completion_tokens = Some(max_completion_tokens);
    }

    pub fn set_parallel_tool_calls(&mut self, parallel_tool_calls: bool) {
        self.chat_request.parallel_tool_calls = parallel_tool_calls;
    }

    pub fn set_presence_penalty(&mut self, presence_penalty: f32) -> Result<(), String> {
        if presence_penalty < -2.0 || presence_penalty > 2.0 {
            return Err("Presence penalty must be between -2.0 and 2.0".to_string());
        }
        self.chat_request.presence_penalty = presence_penalty;
        Ok(())
    }

    pub fn set_reasoning_format(&mut self, reasoning_format: &str){
        self.chat_request.reasoning_format = Some(reasoning_format.to_string());
    }

    pub fn set_response_format(&mut self, response_format: ChatResponseFormat){
        self.chat_request.response_format = Some(response_format);
    }

    pub fn set_seed(&mut self, seed: u64){
        self.chat_request.seed = Some(seed);
    }

    pub fn set_service_tier(&mut self, service_tier: ChatServiceTier){
        self.chat_request.service_tier = Some(service_tier);
    }

    pub fn set_temperature(&mut self, temperature: f32) -> Result<(), String> {
        if temperature < 0.0 || temperature > 2.0 {
            return Err("Temperature must be between 0.0 and 1.0".to_string());
        }
        self.chat_request.temperature = temperature;
        Ok(())
    }

    pub fn set_top_p(&mut self, top_p: f32) -> Result<(), String> {
        if top_p < 0.0 || top_p > 1.0 {
            return Err("Top P must be between 0.0 and 1.0".to_string());
        }
        self.chat_request.top_p = top_p;
        Ok(())
    }

    pub fn set_tool_choice(&mut self, tool_choice: ToolChoice) {
        self.chat_request.tool_choice = Some(tool_choice);
    }

    pub fn add_tool(&mut self, tool: Tool) {
        self.chat_request.tools.push(tool);
    }

    pub fn clear_tools(&mut self) {
        self.chat_request.tools.clear();
    }

    pub fn get_temperature(&self) -> f32 {
        self.chat_request.temperature
    }

    pub fn get_model(&self) -> String {
        self.chat_request.model.clone()
    }

    pub fn get_frequency_penalty(&self) -> f32 {
        self.chat_request.frequency_penalty
    }

    pub fn get_presence_penalty(&self) -> f32 {
        self.chat_request.presence_penalty
    }

    pub fn get_top_p(&self) -> f32 {
        self.chat_request.top_p
    }

    pub async fn send(&self) -> Result<ChatResponse, Box<dyn Error + Send + Sync>> {
        let mut client_builder = reqwest::Client::builder();
        if let Ok(proxy) = env::var("HTTPS_PROXY"){
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy)?);
        }
        let client = client_builder.build()?;
        let body = serde_json::to_string(&self.chat_request)?;
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
            let chat_error = serde_json::from_str::<ChatError>(&raw_response)?;
            return Err(Box::new(chat_error));
        }
        let raw_response = response.text().await?;
        let chat_response = serde_json::from_str::<ChatResponse>(&raw_response)?;
        Ok(chat_response)
    }

    pub async fn stream(&self) -> Result<impl Stream<Item = Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>>> + Unpin, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&self.chat_request)?;
        let response = client
            .post(self.api_url.clone())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(body)
            .send()
            .await?;
    
        if response.status().is_client_error() {
            let raw_response = response.text().await?;
            let chat_error = serde_json::from_str::<ChatError>(&raw_response)?;
            return Err(Box::new(chat_error));
        }
    
        let byte_stream = response
            .bytes_stream()
            .map_err(|e| io::Error::new(ErrorKind::Other, e));
        let stream_reader = StreamReader::new(byte_stream);
        let buf_reader = BufReader::new(stream_reader);
        let lines = FramedRead::new(buf_reader, LinesCodec::new());
    
        let json_stream = lines.filter_map(|line_result| async {
            match line_result {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    let json_str = if let Some(stripped) = trimmed.strip_prefix("data:") {
                        stripped.trim()
                    } else {
                        trimmed
                    };
                    if json_str == "[DONE]" {
                        return None;
                    }
    
                    Some(serde_json::from_str::<ChatResponse>(json_str)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>))
                }
                Err(e) => Some(Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)),
            }
        });
        Ok(json_stream.boxed_unpin())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolCall {
    pub id: String,
    pub r#type: ToolType,
    pub function: ToolCallFunction,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

impl ChatMessage {
    pub fn new(role: ChatRole, content: &str, tool_call_id: Option<String>) -> Self {
        Self {
            role,
            content: Some(content.to_string()),
            tool_calls: None,
            tool_call_id,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ChatResponseFormat {
    JsonObject,
    JsonArray,
    Text,
}

impl ChatResponseFormat {
    pub fn to_string(&self) -> String {
        match self {
            ChatResponseFormat::JsonObject => r#"{ "type": "json_object" }"#.to_string(),
            ChatResponseFormat::JsonArray => r#"{ "type": "json_array" }"#.to_string(),
            ChatResponseFormat::Text => r#"{ "type": "text" }"#.to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ChatServiceTier {
    OnDemand,
    Auto,
    Flex,
}

impl ChatServiceTier {
    pub fn to_string(&self) -> String {
        match self {
            ChatServiceTier::OnDemand => "on_demand".to_string(),
            ChatServiceTier::Auto => "auto".to_string(),
            ChatServiceTier::Flex => "flex".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    frequency_penalty: f32,
    max_completion_tokens: Option<u32>,
    parallel_tool_calls: bool,
    presence_penalty: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ChatResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_tier: Option<ChatServiceTier>,
    stream: bool,
    temperature: f32,
    top_p: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Tool>,
}

impl ChatRequest {
    pub fn new(model: String, messages: Vec<ChatMessage>) -> Self {
        Self { 
            model,
            messages,
            frequency_penalty: 0.0,
            max_completion_tokens: None,
            parallel_tool_calls: true,
            presence_penalty: 0.0,
            reasoning_format: None,
            response_format: None,
            seed: None,
            service_tier: None,
            stream: false,
            temperature: 1.0,
            top_p: 1.0,
            tool_choice: None,
            tools: vec![],
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceValue {
    None,
    Auto,
    Required,
}

/// Represents the object variant for tool choice.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolChoiceObject {
    /// The type of the tool. Currently, only "function" is supported.
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    /// The tool details.
    pub function: ToolFunction,
}

/// Represents the tool details.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ToolFunction {
    /// The name of the function to call.
    pub name: Option<String>,
}

/// An untagged enum that can either be a string (for simple values) or an object (for a specific tool).
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ToolChoice {
    Value(ToolChoiceValue),
    Object(ToolChoiceObject),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    /// A description of what the function does.
    pub function: Function,
    /// The type of the tool. Currently, only "function" is supported.
    #[serde(rename = "type")]
    pub tool_type: ToolType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The parameters the function accepts, described as a JSON Schema object.
    /// Omitting parameters defines a function with an empty parameter list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")] 
pub enum ChatRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
    pub system_fingerprint: String,
    pub x_groq: ChatXGroq,
}

#[derive(Clone, Serialize, Debug)]
pub struct ChatChoice {
    pub index: u64,
    pub message: ChatMessage,
    pub logprobs: Option<String>,
    pub finish_reason: Option<String>,
}
impl<'de> Deserialize<'de> for ChatChoice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a helper with optional delta/message fields.
        #[derive(Deserialize)]
        struct ChatChoiceHelper {
            index: u64,
            #[serde(default)]
            delta: Option<ChatMessage>,
            #[serde(default)]
            message: Option<ChatMessage>,
            logprobs: Option<String>,
            finish_reason: Option<String>,
        }
        
        let helper = ChatChoiceHelper::deserialize(deserializer)?;
        let message = helper.delta.or(helper.message)
            .ok_or_else(|| de::Error::missing_field("delta or message"))?;
        Ok(ChatChoice {
            index: helper.index,
            message,
            logprobs: helper.logprobs,
            finish_reason: helper.finish_reason,
        })
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatUsage {
    pub queue_time: f64,
    pub prompt_tokens: u64,
    pub prompt_time: f64,
    pub completion_tokens: u64,
    pub completion_time: f64,
    pub total_tokens: u64,
    pub total_time: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatXGroq {
    pub id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Error)]
pub struct ChatError {
    error: ChatErrorDetails,
}

impl Display for ChatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chat error: {}", self.error.message)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatErrorDetails {
    message: String,
    r#type: String,
    param: Option<String>,
    code: Option<String>,
}