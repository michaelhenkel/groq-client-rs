use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::responses::{input, mcp, common, tools};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    pub instructions: Instructions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub model: String,
    pub object: Object,
    pub output: Vec<Output>,
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<common::Prompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<common::ServiceTier>,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<tools::ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<tools::Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<common::Truncation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "incomplete")]
    Incomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    OutputMessage(OutputMessage),
    McpToolCall(mcp::McpToolCall),
    McpListTools(mcp::McpListTools),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMessage {
    pub content: Vec<OutputContent>,
    pub id: String,
    pub role: OutputMessageRole,
    pub status: OutputMessageStatus,
    #[serde(rename = "type")]
    pub r#type: OutputMessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputContent {
    OutputText(OutputText),
    Refusal(Refusal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputText {
    pub text: String,
    pub annotations: Vec<serde_json::Value>,
    pub logprobs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "type")]
    pub r#type: OutputTextType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputTextType {
    #[serde(rename = "output_text")]
    OutputText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refusal {
    pub refusal: String,
    #[serde(rename = "type")]
    pub r#type: RefusalType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RefusalType {
    #[serde(rename = "refusal")]
    Refusal,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputMessageType {
    #[serde(rename = "message")]
    Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputMessageRole {
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputMessageStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Object {
    #[serde(rename = "response")]
    #[default]
    Response,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncompleteDetails {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub enum Instructions {
    Text(String),
    InputItemList(Vec<input::InputItem>),
}

impl Default for Instructions {
    fn default() -> Self {
        Instructions::InputItemList(Vec::new())
    }
}