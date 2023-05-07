use crate::config;
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
    pub fn build(self) -> Result<(), &'static str> {
        let api = match self.api {
            Some(args) => args,
            None => return Err("Invalid API key"),
        };

        let _ = config::Config::set_config("api", api);
        Ok(())
    }
}
