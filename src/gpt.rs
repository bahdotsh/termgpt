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

    chat: Option<Vec<String>>,

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
            if let Err(err) = config::Config::set_config("api", api) {
                eprintln!("{:#?}", err);
            }
        }
        if let Some(vecprompt) = self.chat.take() {
            let prompt = vecprompt.join(" ");
            let api = config::Config::get_api_config();
            if let Err(err) = prompt::prompt(&prompt, &api, self.exec) {
                eprintln!("{:#?}", err);
            }
        }

        Ok(())
    }
}
