use std::env;
use reqwest::Client;
use serde_json::{Value, json}; // Updated import

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
                    println!("{}", content);
                }
            }
        }
    }

    Ok(())
}
