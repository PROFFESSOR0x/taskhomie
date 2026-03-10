// OpenAI-compatible API client
// Supports OpenAI, Azure OpenAI, Ollama, LM Studio, vLLM, and other compatible APIs

use crate::agent::AgentMode;
use crate::storage::Usage;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

const DEFAULT_OPENAI_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_MAX_TOKENS: u32 = 16000;

#[derive(Error, Debug)]
pub enum OpenAiApiError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API error: {0}")]
    Api(String),
    #[error("Stream error: {0}")]
    Stream(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OpenAiContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    Image { image_url: OpenAiImageUrl },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiMessage {
    pub role: String,
    pub content: OpenAiContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiContent {
    Text(String),
    Blocks(Vec<OpenAiContentBlock>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiToolFunction {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiTool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: OpenAiToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: OpenAiFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiFunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiResponseMessage {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenAiToolCall>>,
}

#[derive(Debug, Serialize)]
pub struct OpenAiRequest {
    pub model: String,
    pub messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<OpenAiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiResponse {
    pub id: String,
    pub choices: Vec<OpenAiChoice>,
    pub usage: Option<OpenAiUsage>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChoice {
    pub message: OpenAiResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OpenAiUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// SSE streaming event
#[derive(Debug, Deserialize)]
pub struct OpenAiStreamChunk {
    pub id: Option<String>,
    pub choices: Vec<OpenAiStreamChoice>,
    pub usage: Option<OpenAiUsage>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiStreamChoice {
    pub delta: OpenAiStreamDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct OpenAiStreamDelta {
    pub role: Option<String>,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<OpenAiStreamToolCall>>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiStreamToolCall {
    pub index: usize,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub call_type: Option<String>,
    pub function: Option<OpenAiStreamFunction>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiStreamFunction {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

// streaming event types for agent
#[derive(Debug, Clone)]
pub enum OpenAiStreamEvent {
    TextDelta { text: String },
    ToolUseStart { id: String, name: String },
    ToolUseDelta { index: usize, arguments: String },
    MessageStop,
}

pub struct OpenAiClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl OpenAiClient {
    pub fn new(api_key: String, base_url: Option<String>, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| DEFAULT_OPENAI_URL.to_string()),
            model,
        }
    }

    fn build_tools(&self, mode: AgentMode) -> Vec<OpenAiTool> {
        let mut tools = Vec::new();

        match mode {
            AgentMode::Computer => {
                // Computer control tool
                tools.push(OpenAiTool {
                    tool_type: "function".to_string(),
                    function: OpenAiToolFunction {
                        name: "computer".to_string(),
                        description: Some("Control the computer via mouse and keyboard".to_string()),
                        parameters: serde_json::json!({
                            "type": "object",
                            "properties": {
                                "action": {
                                    "type": "string",
                                    "enum": ["screenshot", "mouse_move", "left_click", "right_click", "double_click", "triple_click", "middle_click", "left_click_drag", "type", "key", "scroll", "wait", "left_mouse_down", "left_mouse_up", "hold_key", "zoom"],
                                    "description": "The action to perform"
                                },
                                "coordinate": {
                                    "type": "array",
                                    "items": {"type": "integer"},
                                    "description": "[x, y] coordinates in 1280x800 space"
                                },
                                "start_coordinate": {
                                    "type": "array",
                                    "items": {"type": "integer"},
                                    "description": "Starting [x, y] for drag actions"
                                },
                                "text": {
                                    "type": "string",
                                    "description": "Text to type or key to press"
                                },
                                "scroll_direction": {
                                    "type": "string",
                                    "enum": ["up", "down", "left", "right"]
                                },
                                "scroll_amount": {
                                    "type": "integer"
                                },
                                "key": {
                                    "type": "string",
                                    "description": "Key to hold for hold_key action"
                                },
                                "region": {
                                    "type": "array",
                                    "items": {"type": "integer"},
                                    "description": "[x1, y1, x2, y2] region for zoom"
                                }
                            },
                            "required": ["action"]
                        }),
                    },
                });
            }
            AgentMode::Browser => {
                // Browser automation tools
                tools.extend(self.build_browser_tools());
            }
        }

        // Bash tool
        tools.push(OpenAiTool {
            tool_type: "function".to_string(),
            function: OpenAiToolFunction {
                name: "bash".to_string(),
                description: Some("Execute bash commands on the system".to_string()),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The bash command to execute"
                        }
                    },
                    "required": ["command"]
                }),
            },
        });

        // Speak tool for voice mode
        tools.push(OpenAiTool {
            tool_type: "function".to_string(),
            function: OpenAiToolFunction {
                name: "speak".to_string(),
                description: Some("Speak to the user via text-to-speech".to_string()),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "text": {
                            "type": "string",
                            "description": "Natural spoken text to convert to speech"
                        }
                    },
                    "required": ["text"]
                }),
            },
        });

        tools
    }

    fn build_browser_tools(&self) -> Vec<OpenAiTool> {
        vec![
            OpenAiTool {
                tool_type: "function".to_string(),
                function: OpenAiToolFunction {
                    name: "see_page".to_string(),
                    description: Some("Get page structure and elements".to_string()),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "screenshot": {"type": "boolean", "description": "Include screenshot"},
                            "list_tabs": {"type": "boolean", "description": "List all open tabs"},
                            "verbose": {"type": "boolean", "description": "Include detailed info"}
                        }
                    }),
                },
            },
            OpenAiTool {
                tool_type: "function".to_string(),
                function: OpenAiToolFunction {
                    name: "page_action".to_string(),
                    description: Some("Interact with page elements".to_string()),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "click": {"type": "string", "description": "Element UID to click"},
                            "double_click": {"type": "string"},
                            "type_into": {"type": "string"},
                            "text": {"type": "string"},
                            "hover": {"type": "string"},
                            "press_key": {"type": "string"},
                            "scroll": {"type": "string"},
                            "scroll_pixels": {"type": "integer"},
                            "dialog": {"type": "string", "enum": ["accept", "dismiss"]},
                            "dialog_text": {"type": "string"}
                        }
                    }),
                },
            },
            OpenAiTool {
                tool_type: "function".to_string(),
                function: OpenAiToolFunction {
                    name: "browser_navigate".to_string(),
                    description: Some("Navigate browser or manage tabs".to_string()),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "go_to_url": {"type": "string"},
                            "go_back": {"type": "boolean"},
                            "go_forward": {"type": "boolean"},
                            "reload": {"type": "boolean"},
                            "reload_skip_cache": {"type": "boolean"},
                            "open_new_tab": {"type": "string"},
                            "switch_to_tab": {"type": "integer"},
                            "close_tab": {"type": "integer"},
                            "wait_for_text": {"type": "string"},
                            "wait_timeout_ms": {"type": "integer"}
                        }
                    }),
                },
            },
        ]
    }

    pub async fn send_message_streaming(
        &self,
        messages: Vec<OpenAiMessage>,
        event_tx: mpsc::UnboundedSender<OpenAiStreamEvent>,
        mode: AgentMode,
        _voice_mode: bool,
    ) -> Result<OpenAiApiResult, OpenAiApiError> {
        let tools = self.build_tools(mode);

        let request = OpenAiRequest {
            model: self.model.clone(),
            messages,
            tools: Some(tools),
            tool_choice: None, // Auto-select tools
            max_tokens: Some(DEFAULT_MAX_TOKENS),
            stream: Some(true),
            temperature: Some(0.7),
        };

        let response = self
            .client
            .post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            return Err(OpenAiApiError::Api(format!("HTTP {}: {}", status, body)));
        }

        // Parse SSE stream
        let mut tool_calls: Vec<Option<OpenAiToolCall>> = Vec::new();
        let mut content_text: String = String::new();
        let mut usage = Usage::default();
        let mut buffer = String::new();

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(newline_pos) = buffer.find('\n') {
                let line = buffer[..newline_pos].trim().to_string();
                buffer = buffer[newline_pos + 1..].to_string();

                if !line.starts_with("data: ") {
                    continue;
                }

                let data = &line[6..];
                if data == "[DONE]" {
                    break;
                }

                if let Ok(event) = serde_json::from_str::<OpenAiStreamChunk>(data) {
                    // Capture usage
                    if let Some(u) = &event.usage {
                        usage.input_tokens = u.prompt_tokens;
                        usage.output_tokens = u.completion_tokens;
                    }

                    if let Some(choice) = event.choices.first() {
                        // Text content
                        if let Some(ref text) = choice.delta.content {
                            if !text.is_empty() {
                                content_text.push_str(text);
                                let _ = event_tx.send(OpenAiStreamEvent::TextDelta {
                                    text: text.to_string(),
                                });
                            }
                        }

                        // Tool calls
                        if let Some(ref tool_calls_delta) = choice.delta.tool_calls {
                            for tc in tool_calls_delta {
                                // Ensure vector is big enough
                                while tool_calls.len() <= tc.index {
                                    tool_calls.push(None);
                                }

                                // Initialize if needed
                                if tool_calls[tc.index].is_none() {
                                    tool_calls[tc.index] = Some(OpenAiToolCall {
                                        id: tc.id.clone().unwrap_or_default(),
                                        call_type: tc.call_type.clone().unwrap_or_else(|| "function".to_string()),
                                        function: OpenAiFunctionCall {
                                            name: String::new(),
                                            arguments: String::new(),
                                        },
                                    });
                                }

                                if let Some(ref tool) = tool_calls[tc.index] {
                                    let mut tool_clone = tool.clone();
                                    
                                    if let Some(ref func) = tc.function {
                                        if let Some(ref name) = func.name {
                                            tool_clone.function.name = name.clone();
                                            let _ = event_tx.send(OpenAiStreamEvent::ToolUseStart {
                                                id: tool.id.clone(),
                                                name: name.clone(),
                                            });
                                        }
                                        if let Some(ref args) = func.arguments {
                                            tool_clone.function.arguments.push_str(args);
                                            let _ = event_tx.send(OpenAiStreamEvent::ToolUseDelta {
                                                index: tc.index,
                                                arguments: args.to_string(),
                                            });
                                        }
                                    }
                                    tool_calls[tc.index] = Some(tool_clone);
                                }
                            }
                        }

                        // Check for finish
                        if choice.finish_reason.as_deref() == Some("stop")
                            || choice.finish_reason.as_deref() == Some("tool_calls")
                        {
                            let _ = event_tx.send(OpenAiStreamEvent::MessageStop);
                        }
                    }
                }
            }
        }

        // Build final response
        let mut content_blocks: Vec<OpenAiContentBlock> = Vec::new();
        let mut tool_uses: Vec<OpenAiToolCall> = Vec::new();

        if !content_text.is_empty() {
            content_blocks.push(OpenAiContentBlock::Text {
                text: content_text,
            });
        }

        for tool_call in tool_calls.into_iter().flatten() {
            tool_uses.push(tool_call);
        }

        Ok(OpenAiApiResult {
            content: content_blocks,
            tool_calls: tool_uses,
            usage,
        })
    }
}

// API result with content and usage
#[derive(Debug)]
pub struct OpenAiApiResult {
    pub content: Vec<OpenAiContentBlock>,
    pub tool_calls: Vec<OpenAiToolCall>,
    pub usage: Usage,
}
