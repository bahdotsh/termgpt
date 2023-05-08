use clap::Parser;
use termgpt::gpt::GPT;

fn main() {
    GPT::run();
    let gpt = GPT::parse();
    let _ = GPT::build(gpt);
}
