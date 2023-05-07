use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Gokul <@bahdotsh>",
    version,
    about = "Cli tool to interact with chatgpt"
)]
pub struct GPT {
    #[clap(long)]
    api: Option<String>,

    #[clap(long)]
    chat: Option<String>,

    #[clap(long, short)]
    exec: bool,
}
