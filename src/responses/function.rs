use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Function {
    pub name: String,
    pub parameters: serde_json::Value,
    #[serde(default = "default_strict")]
    pub strict: bool,
    pub r#type: FunctionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn default_strict() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum FunctionType {
    #[serde(rename = "function")]
    #[default]
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FunctionToolCall {
    pub arguments: serde_json::Value,
    pub call_id: String,
    pub name: String,
    pub r#type: FunctionToolCallType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionToolCallStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum FunctionToolCallType {
    #[serde(rename = "function_call")]
    #[default]
    FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionToolCallOutput {
    pub call_id: String,
    pub output: serde_json::Value,
    pub r#type: FunctionToolCallOutputType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionToolCallStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionToolCallOutputType {
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedToolsToolFunction {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: AllowedToolsToolFunctionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedToolsToolFunctionType {
    #[serde(rename = "function")]
    Function,
}