use serde::{Deserialize, Serialize};
use crate::responses::{mcp, function};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Input {
    TextInput(String),
    InputItemList(Vec<InputItem>),
    ItemReference(ItemReference),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemReference {
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: ItemReferenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemReferenceType {
    #[serde(rename = "item_reference")]
    ItemReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputItem {
    InputMessage(InputMessage),
    Item(Item)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    InputMessage(InputMessage),
    OutputMessage(OutputMessage),
    McpListTools(mcp::McpListTools),
    McpToolCall(mcp::McpToolCall),
    FunctionToolCall(function::FunctionToolCall),
    FunctionToolCallOutput(function::FunctionToolCallOutput),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputMessage {
    pub content: Vec<OutputContent>,
    pub id: String,
    pub role: Role,
    pub status: ItemStatus,
    #[serde(rename = "type")]
    pub r#type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputContent {
    OutputText(OutputText),
    Refusal(Refusal),
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
pub struct OutputText {
    pub text: String,
    pub annotations: Vec<serde_json::Value>,
    pub logprobs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "type")]
    pub r#type: OutputTextType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputMessage {
    pub content: InputContent,
    pub role: Role,
    pub status: Option<ItemStatus>,
    #[serde(rename = "type")]
    pub r#type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ItemStatus {
    #[default]
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputContent {
    TextInput(String),
    ItemInputContentList(Vec<InputText>),
}

impl Default for InputContent {
    fn default() -> Self {
        Self::TextInput("".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputText {
    pub text: String,
    #[serde(rename = "type")]
    pub r#type: InputTextType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputTextType {
    #[serde(rename = "input_text")]
    InputText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputTextType {
    #[serde(rename = "output_text")]
    OutputText,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MessageType {
    #[serde(rename = "message")]
    #[default]
    Message,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Role {
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}

