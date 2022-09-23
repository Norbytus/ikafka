use clap::Parser;
use colored::Colorize;
use commands::{Commands, CommandExecute};

mod commands;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_hello_message();

    let args = commands::Cli::parse();

    match args.commands {
        Commands::ReadMessage(command) => {
            command.run()
        },
        Commands::ListOfTopics(command) => {
            command.run()
        }
    }
}

fn print_hello_message() {
    println!("{} {} {}", "Stand".blue(), "with".yellow(), "Satan".red());
}
