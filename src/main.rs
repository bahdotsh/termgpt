use clap::Parser;
use rgpt::gpt::GPT;

fn main() {
    GPT::run();
    let gpt = GPT::parse();
    let _ = GPT::build(gpt);
}
