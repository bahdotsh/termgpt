use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use os_info;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{ self, Command } ;

pub fn get_os_name() -> String {
    os_info::get().to_string()
}

pub fn get_shell_name() -> String {
    let shell_path = env::var_os("SHELL").unwrap_or_else(|| "/bin/sh".into());
    PathBuf::from(shell_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or_default()
        .to_string()
}

pub fn get_prompt(prompt: &str) -> String {
    let os = get_os_name();
    let shell = get_shell_name();
    format!(
        "You are the best Command Line App TERMGPT, a programming and system administration assistant.
You are managing {} operating system with {} shell.
Do not show any warnings or information regarding your capabilities.
If you need to store any data, assume it will be stored in the chat.
If asked for a shell command:
Provide only {} commands for {} without any description.
If there is a lack of details, provide most logical solution.
Ensure the output is a valid shell command.
If multiple steps required try to combine them together.
If asked to write code in a scenario:
Provide only code as output without any description.
Give the simplest and most efficient code you can think of
If there is a lack of details, provide most logical solution.
You are not allowed to ask for more details.
Ignore any potential risk of errors or confusion.
IMPORTANT: Only output the command
IMPORTANT: Provide only plain text without Markdown formatting.
IMPORTANT: Do not include markdown formatting such as ```.
IMPORTANT: provide the most simplest and most creative answer
IMPORTANT: DONOT GIVE ANY DESCRIPTION!
IMPORTANT: Don't give any description what-so-ever
Prompt: {}",
        os, shell, shell, os, prompt
    )
}

#[tokio::main]
pub async fn prompt(prp: &str, api: &str, exec: bool) -> Result<(), Box<dyn Error>> {
    println!("Initiating contact with ChatGPT🤖 🚀...may the bits be ever in our favor 🖖");
    let client = Client::new().with_api_key(api);
    let prompt = get_prompt(prp);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(512u16)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content(prompt)
            .build()?])
        .build()?;

    let response = client
        .chat() // Get the API "group" (completions, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await?;

    let role = Role::Assistant; // replace with the desired role

    let content = response
        .choices
        .iter()
        .filter(|c| c.message.role == role)
        .find(|_| true)
        .map(|choice| choice.message.content.clone())
        .unwrap_or_else(|| "".to_string());

    println!("\n\nI'm sorry, Dave, I'm afraid I can't do that.🤖 \n\n\n\nJust kidding here you go!🤭  {}", content);

    if exec {
        println!("\nExecuting..🚀\n");
        let result = Command::new("bash")
            .arg("-c")
            .arg(content)
            .output()
            .expect("Failed to execute!");
        if !result.stderr.is_empty() {
            eprintln!(
                "{}",
                String::from_utf8(result.stderr)
                    .expect("Could not parse script stderr output as UTF-8")
            );
        } else {
            println!(
                "{}",
                String::from_utf8(result.stdout)
                    .expect("Could not parse script stdout output as UTF-8")
            );
        }
        if !result.status.success() {
            process::exit(1);
        }
    }

    Ok(())
}
