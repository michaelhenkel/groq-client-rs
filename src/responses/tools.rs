use serde::{Deserialize, Serialize};
use crate::responses::{mcp, function};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tool {
    McpTool(mcp::McpTool),
    Function(function::Function),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolChoice {
    ToolChoiceMode(ToolChoiceMode),
    AllowedTools(AllowedTools),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedTools {
    pub mode: AllowedToolsToolChoiceMode,
    pub tools: Vec<AllowedToolsTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AllowedToolsType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedToolsType {
    #[serde(rename = "allowed_tools")]
    AllowedTools,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedToolsTool {
    AllowedToolsToolMcp(mcp::AllowedToolsToolMcp),
    AllowedToolsToolFunction(function::AllowedToolsToolFunction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedToolsToolChoiceMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolChoiceMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "required")]
    Required,
}