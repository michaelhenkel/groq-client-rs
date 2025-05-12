use std::{collections::HashMap, env, error::Error};

use groq_client_rs::chat::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let mut function_map: HashMap<String, fn(String) -> Result<String, Box<dyn Error + Send + Sync>>> = HashMap::new();
    function_map.insert("run_tool1".to_string(), run_tool1);
    function_map.insert("run_tool2".to_string(), run_tool2);

    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY is not set");
    let mut chat = Chat::new(
        api_key,
        "deepseek-r1-distill-llama-70b".to_string(),
    );

    chat.add_chat_message(ChatMessage::new(
        ChatRole::System,
        r#"You are a network automation expert. 
        You generate network configuration commands to achieve a given goal.
        The network configurations have to match the network device type.
        Output a function call with the correct network device configuration.
        The configuration must match the syntax of the specific network device and model.
        It must be a function from the list of tools.
        Functions must be named exactly as in the list of tools.
        Functions are written in Rust.
        Don't output any commentary or markdown.
        "#,
        None
    ));

    chat.add_chat_message(ChatMessage::new(
        ChatRole::User,
        "configure unnumbered bgp on the cisco nexus 9000 switch using interface Ethernet1/1",
        None
    ));

    
    chat.set_tool_choice(ToolChoice::Value(ToolChoiceValue::Required));
    
     
    chat.add_tool(Tool {
        function: Function {
            name: Some("configure_cisco_switch".to_string()),
            description: Some("Sends a configuration to a Cisco Nexus 9000 switch".to_string()),
            parameters: Some(json!({
                "configuration": {
                    "type": "string",
                    "description": "The configuration to send to the switch"
                }
            })),
        },
        tool_type: ToolType::Function,
    });

    chat.add_tool(Tool {
        function: Function {
            name: Some("configure_juniper_switch".to_string()),
            description: Some("Sends a configuration to a Juniper QFX switch".to_string()),
            parameters: Some(json!({
                "configuration": {  
                    "type": "string",
                    "description": "The configuration to send to the switch"
                }
            })),
        },
        tool_type: ToolType::Function,
    });
    

    loop {
        let response = chat.send().await?;
        if let Some(tool_calls) = &response.choices[0].message.tool_calls {
            for tool_call in tool_calls {
                let function_name = tool_call.function.name.clone();
                if let Some(function) = function_map.get(&function_name) {
                    let result = function(tool_call.function.arguments.clone())?;
                    let tool_response = ChatMessage::new(
                        ChatRole::Tool,
                        &result,
                        Some(tool_call.id.clone()),
                    );
                    chat.add_chat_message(tool_response);
                }
            }
        } else {
            println!("{:?}", response.choices[0].message.content);
            break;
        }
    }

    
    Ok(())
}

fn run_tool1(configuration: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    println!("{}", configuration);
    Ok("result from tool1".to_string())
}

fn run_tool2(configuration: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    println!("{}", configuration);
    Ok("result from tool2".to_string())
}