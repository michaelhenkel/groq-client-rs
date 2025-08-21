use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tool {
    McpTool(McpTool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub server_label: String,
    pub server_url: String,
    #[serde(rename = "type")]
    pub r#type: McpToolType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<McpToolAllowedTools>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpToolAllowedTools {
    McpAllowedTools(Vec<String>),
    McpAllowedToolsFilter(McpAllowedToolsFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpAllowedToolsFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpToolType {
    #[serde(rename = "mcp")]
    Mcp,
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
    AllowedToolsToolMcp(AllowedToolsToolMcp),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedToolsToolMcp {
    pub server_label: String,
    #[serde(rename = "type")]
    pub r#type: AllowedToolsToolMcpType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedToolsToolMcpType {
    #[serde(rename = "mcp")]
    Mcp,
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