mod commands;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
    Add,
    Commit,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add => {
            commands::add::add();
        }
        Command::Commit => {
            println!("fn main Commit");
        }
        Command::Init => {
            commands::init::init();
        }
    }
}
