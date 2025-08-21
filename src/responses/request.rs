use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::responses::{input, tools, common};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<input::Input>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<common::Prompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<common::ServiceTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default = "default_temperature")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<tools::ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<tools::Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default = "default_top_p")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<common::Truncation>,
}

fn default_top_p() -> Option<f64> {
    Some(1.0)
}

fn default_temperature() -> Option<f64> {
    Some(1.0)
}

impl Request {
    pub fn new(model: String, input: Option<input::Input>) -> Self {
        Self { model, input, ..Default::default() }
    }
}