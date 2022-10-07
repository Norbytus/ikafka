use self::{list_of_topics::ListOfTopics, message_regex::ReadMessages};
use chrono::Local;
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use regex::Regex;
use std::ops::Range;

const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

mod list_of_topics;
mod message_regex;
mod printers;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ReadMessage(ReadMessages),
    ListOfTopics(ListOfTopics),
}

pub trait CommandExecute {
    fn run(self) -> Result<(), Box<dyn std::error::Error>>;
}
