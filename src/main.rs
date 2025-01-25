use std::env;
use reqwest::Client;
use serde_json::Value; // Removed unused json import
use synoptic::{Highlighter, TokOpt};
use lliw::Fg;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the OpenAI API key from the environment variable
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");

    // Get user message from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <message>", args[0]);
        std::process::exit(1);
    }
    let user_message = &args[1];

    let payload = serde_json::json!({
        "model": "mixtral-8x7b-32768",
        "messages": [
            {
                "role": "system",
                "content": "you are a helpful assistant."
            },
            {
                "role": "user",
                "content": user_message
            }
        ],
        "temperature": 0.5,
        "max_tokens": 1024,
        "top_p": 1,
        "stop": null,
        "stream": false
    }).to_string();

    // Create a reqwest client
    let client = Client::new();

    // Make the POST request to the OpenAI API
    let response = client.post("https://api.groq.com/openai/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(payload)
        .send()
        .await?;

    // Extract the response body as a string
    let body = response.text().await?;

    // Deserialize the JSON response
    let json: Value = serde_json::from_str(&body)?;

    // Extract the contents from the JSON response
    if let Some(choices) = json["choices"].as_array() {
        for choice in choices {
            if let Some(message) = choice["message"].as_object() {
                if let Some(content) = message["content"].as_str() {
                    // Use synoptic to highlight markdown
                    let mut h = synoptic::from_extension("md", 4).unwrap_or_else(|| Highlighter::new(4));
                    
                    // Split content into lines
                    let lines: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();
                    
                    // Run the highlighter
                    h.run(&lines);
                    
                    // Render the output
                    for (line_number, line) in lines.iter().enumerate() {
                        for token in h.line(line_number, line) {
                            match token {
                                TokOpt::Some(text, kind) => print!("{}{text}{}", colour(&kind), Fg::Reset),
                                TokOpt::None(text) => print!("{text}"),
                            }
                        }
                        println!(); // Newline at end of each line
                    }
                }
            }
        }
    }

    Ok(())
}

// Colour function for markdown tokens
fn colour(name: &str) -> Fg {
    match name {
        "heading" => Fg::Blue,
        "block" => Fg::Yellow,
        "link" => Fg::Purple,
        "bold" => Fg::Yellow,
        "italic" => Fg::LightPurple,
        "strikethrough" => Fg::LightBlack,
        "quote" => Fg::LightBlue,
        "image" => Fg::Red,  // Replaced Magenta with Red
        "math" => Fg::Cyan,
        "comment" => Fg::LightBlack,
        "linebreak" => Fg::LightBlack,
        "list" => Fg::Red,
        _ => Fg::Reset,
    }
}
