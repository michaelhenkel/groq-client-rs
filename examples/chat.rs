use std::{collections::HashMap, env, error::Error};

use groq_rs::chat::*;
use serde_json::json;
//use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let mut function_map: HashMap<String, fn(String)> = HashMap::new();
    function_map.insert("configure_cisco_switch".to_string(), configure_cisco_switch);
    function_map.insert("configure_juniper_switch".to_string(), configure_juniper_switch);

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
        "#
    ));

    chat.add_chat_message(ChatMessage::new(
        ChatRole::User,
        "configure unnumbered bgp on the cisco nexus 9000 switch using interface Ethernet1/1"
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
    


    let response = chat.send().await?;
    if let Some(tool_calls) = &response.choices[0].message.tool_calls {
        for tool_call in tool_calls {
            let function_name = tool_call.function.name.clone();
            if let Some(function) = function_map.get(&function_name) {
                function(tool_call.function.arguments.clone());
            }
        }
    }

    /*
    let mut stream = chat.stream().await?;
    while let Some(chat_response) = stream.try_next().await? {
        println!("{}", chat_response.choices[0].message.content);
    }
    */
    
    Ok(())
}

fn configure_cisco_switch(configuration: String) {
    println!("{}", configuration);
}

fn configure_juniper_switch(configuration: String) {
    println!("{}", configuration);
}