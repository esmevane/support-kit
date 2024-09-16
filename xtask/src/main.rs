mod cli;
mod runnable;
mod tasks;

use clap::Parser;
use runnable::Runnable;

use cli::Cli;

fn main() {
    Cli::parse().run();
}
