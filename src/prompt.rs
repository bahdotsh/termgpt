use async_openai::{types::CreateCompletionRequestArgs, Client};
use std::error::Error;

#[tokio::main]
pub async fn prompt(prp: &str, api: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new().with_api_key(api);

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(prp)
        .max_tokens(40_u16)
        .build()?;

    let response = client
        .completions() // Get the API "group" (completions, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await?;

    println!("\nResponse (single):\n");
    for choice in response.choices {
        println!("{}", choice.text);
    }

    Ok(())
}
