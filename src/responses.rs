use std::fmt::{self, Display};
use std::{collections::HashMap, env};
use std::error::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Responses {
    api_key: String,
    api_url: String,
    responses_request: ResponsesRequest,
}
impl Responses {
    pub fn new(api_key: String, model: String, api_url: Option<String>) -> Self {
        Self {
            api_key,
            api_url: api_url.unwrap_or("https://api.groq.com/openai/v1/responses".to_string()),
            responses_request: ResponsesRequest::new(model, Input::Text("".to_string())),
        }
    }

    pub async fn send(&self) -> Result<ResponsesResponse, Box<dyn Error + Send + Sync>> {
        let mut client_builder = reqwest::Client::builder();
        if let Ok(proxy) = env::var("HTTPS_PROXY"){
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy)?);
        }
        let client = client_builder.build()?;
        let body = serde_json::to_string(&self.responses_request)?;
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
        let responses_response = serde_json::from_str::<ResponsesResponse>(&raw_response)?;
        Ok(responses_response)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsesRequest {
    /// Required: ID of the model to use
    pub model: String,

    /// Required by your earlier spec: text or array items
    pub input: Input,

    /// Optional system/developer preface message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Optional max output tokens (visible + reasoning)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,

    /// Optional metadata (≤16 pairs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Optional: enable parallel execution of tool calls (default true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Optional: reasoning configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,

    /// Optional: service tier (default auto)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// Optional: response storage flag (currently only false/null supported)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// Optional: SSE streaming (default false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Optional: randomness 0–2 (default 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Optional: response format configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfig>,

    /// Optional: tool selection control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Optional: function tools (≤128)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Optional: nucleus sampling 0–1 (default 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// Optional: context truncation (default disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<Truncation>,

    /// Optional: end-user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

// ---- Reasoning ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConfig {
    /// Defaults to medium
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<Effort>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Effort {
    Low,
    Medium,
    High,
}

// ---- Service tier ----

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
}

// ---- Text formatting ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextConfig {
    /// Spec is polymorphic. Keep flexible with `serde_json::Value` while
    /// still allowing common `{ "type": "text" }` or JSON schema objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<serde_json::Value>,
}

// ---- Tooling ----

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// "none" | "auto" | "required"
    Mode(ToolChoiceMode),
    /// Force a specific function call:
    /// { "type": "function", "function": { "name": "my_function" } }
    Function {
        #[serde(rename = "type")]
        r#type: ToolType, // must be "function"
        function: FunctionRef,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRef {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub r#type: ToolType, // "function"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // required for function tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema for parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    /// Strict schema adherence for generated function call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

// ---- Truncation ----

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Truncation {
    Auto,
    Disabled,
}

/// Top-level `input` can be either a single string or an array of items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    Text(String),
    Items(Vec<InputItem>),
}

/// An item inside `input` can be:
/// - an "easy" message (content string/array, optional role),
/// - a typed message (`type: "message"` with content array),
/// - an item reference,
/// - a function call,
/// - a function call output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputItem {
    EasyMessage(EasyInputMessage),
    Message(InputMessage),
    ItemReference(ItemReference),
    FunctionCall(FunctionCall),
    FunctionCallOutput(FunctionCallOutput),
}

/// "Easy input message": content as string or array; role may be user/assistant/system/developer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EasyInputMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<MessageContent>, // string or array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

/// Typed input message: `type` is "message"; role allowed: user/system/developer
/// (assistant is *not* supported here per spec).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputMessage {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MessageType>, // Always Some(MessageType::Message) when set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InputContentItem>>,
}

/// Item reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ItemReferenceType>, // Always Some(ItemReferenceType::ItemReference) when set
}

/// Function call
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionCall {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FunctionCallType>, // Always Some(FunctionCallType::FunctionCall) when set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// JSON string of arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// Function call output
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionCallOutput {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FunctionCallOutputType>, // Always Some(FunctionCallOutputType::FunctionCallOutput) when set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// JSON string of output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// Content for "easy" messages can be a single string or a list of items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Items(Vec<InputContentItem>),
}

/// Content item inside a typed message's `content` array.
/// Spec didn’t enumerate the item schemas here, so keep it open.
/// You can replace this with concrete variants later.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputContentItem {
    Text(String),
    Obj(serde_json::Value),
}

/// Roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Developer,
}

/// Item status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    InProgress,
    Completed,
    Incomplete,
}

/// Fixed `type` enums
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemReferenceType {
    ItemReference,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCallType {
    FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCallOutputType {
    FunctionCallOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponsesResponse {
    pub id: String,
    pub object: String,
    pub status: String,
    pub created_at: u64,

    pub output: Vec<ResponseOutput>,

    pub previous_response_id: Option<String>,
    pub model: String,

    pub reasoning: ReasoningFields,

    pub max_output_tokens: Option<u64>,
    pub instructions: Option<String>,

    /// Present in responses (e.g., { "format": { "type": "text" } })
    pub text: Option<TextResponse>,

    /// Returned tools; schema can vary across providers
    pub tools: Vec<serde_json::Value>,

    /// In responses this may be a string like "auto", or an object in other cases.
    /// Keep flexible.
    pub tool_choice: serde_json::Value,

    pub truncation: String,

    pub metadata: HashMap<String, serde_json::Value>,

    pub temperature: f64,
    pub top_p: f64,

    pub user: Option<String>,

    pub service_tier: String,

    pub error: Option<serde_json::Value>,
    pub incomplete_details: Option<serde_json::Value>,

    pub usage: Usage,

    pub parallel_tool_calls: bool,
    pub store: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseOutput {
    #[serde(rename = "type")]
    pub r#type: String, // "message"
    pub id: String,
    pub status: String, // "completed" | "in_progress" | "incomplete"
    pub role: String,   // "assistant" (for output items)
    pub content: Vec<OutputContentItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputContentItem {
    #[serde(rename = "type")]
    pub r#type: String, // e.g., "output_text" (may expand in future)
    pub text: String,
    /// Annotations are provider/model-specific; keep generic.
    pub annotations: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReasoningFields {
    pub effort: Option<String>,  // null | "low" | "medium" | "high"
    pub summary: Option<String>, // null or summary text
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextResponse {
    pub format: TextFormatWrapper,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextFormatWrapper {
    #[serde(rename = "type")]
    pub r#type: String, // e.g., "text"
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Usage {
    pub input_tokens: u64,
    pub input_tokens_details: InputTokensDetails,
    pub output_tokens: u64,
    pub output_tokens_details: OutputTokensDetails,
    pub total_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputTokensDetails {
    pub cached_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputTokensDetails {
    pub reasoning_tokens: u64,
}

impl ResponsesRequest {
    pub fn new(model: String, input: Input) -> Self {
        Self {
            model,
            input,
            instructions: None,
            max_output_tokens: None,
            metadata: None,
            parallel_tool_calls: None,
            reasoning: None,
            service_tier: None,
            store: None,
            stream: None,
            temperature: None,
            text: None,
            tool_choice: None,
            tools: None,
            top_p: None,
            truncation: None,
            user: None,
        }
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
