use clap::Parser;
use rgpt::gpt::GPT;

fn main() {
    GPT::run();
    GPT::build(GPT::parse());
}
