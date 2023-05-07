use clap::Parser;
use rgpt::gpt::GPT;

fn main() {
    GPT::run();
    let _ = GPT::build(GPT::parse());
}
