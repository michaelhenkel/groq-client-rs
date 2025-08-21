use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpTool {
    pub name: String,
    pub server_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
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


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum McpToolType {
    #[serde(rename = "mcp")]
    #[default]
    Mcp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCall {
    pub arguments: String,
    pub id: String,
    pub name: String,
    pub server_label: String,
    #[serde(rename = "type")]
    pub r#type: McpToolCallType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpToolCallType {
    #[serde(rename = "mcp_call")]
    McpToolCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpListTools {
    pub id: String,
    pub server_label: String,
    pub tools: Vec<McpListTool>,
    #[serde(rename = "type")]
    pub r#type: McpListToolsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpListTool {
    pub input_schema: serde_json::Value,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpListToolsType {
    #[serde(rename = "mcp_list_tools")]
    McpListTools,
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