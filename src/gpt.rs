use crate::config;
use crate::prompt;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct GPT {
    /// Your API key for OpenAPI
    #[arg(long)]
    api: Option<String>,

    #[arg(long)]
    chat: Option<String>,

    /// Execute the commands generated
    #[clap(long, short)]
    exec: bool,
}

impl GPT {
    pub fn run() {
        config::Config::make_config();
    }
    pub fn build(mut self) -> Result<(), &'static str> {
        if let Some(api) = self.api.take() {
            if let err = config::Config::set_config("api", api) {
                println!("{:#?}", err);
            }
        }
        if let Some(prompt) = self.chat.take() {
            let api = config::Config::get_api_config();
            if let err = prompt::prompt(&prompt, &api) {
                println!("{:#?}", err);
            }
        }

        Ok(())
    }
}
