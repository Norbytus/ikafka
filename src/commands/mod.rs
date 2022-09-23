use clap::{Parser, Subcommand, Args};
use colored::Colorize;
use std::ops::{RangeBounds, Range};
use chrono::Local;
use self::{message_regex::ReadMessages, list_of_topics::ListOfTopics};
use regex::Regex;

const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

mod message_regex;
mod list_of_topics;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ReadMessage (ReadMessages),
    ListOfTopics (ListOfTopics)
}

pub trait CommandExecute {
    fn run(self) -> Result<(), Box<dyn std::error::Error>>;
}

fn print_matched_text(regex: &Regex, m: &mut String) {
    let matches: Vec<Range<usize>> = regex.find_iter(m.as_str())
        .into_iter()
        .map(|mat| mat.start()..mat.end())
        .collect();

    for mat in matches {
        let  word = &m[mat.clone()];

        m.replace_range(mat, word.red().to_string().as_str());
    }

    print_message(m);
}

fn print_message(m: &str) {
    let date = Local::now().format(DATE_FORMAT).to_string();
    println!("[{}] - {}", date, m);
}

fn print_matched_text_witout_date(regex: &Regex, m: &mut String) {
    let matches: Vec<Range<usize>> = regex.find_iter(m.as_str())
        .into_iter()
        .map(|mat| mat.start()..mat.end())
        .collect();

    for mat in matches {
        let  word = &m[mat.clone()];

        m.replace_range(mat, word.red().to_string().as_str());
    }

    print_message_withot_date(m);
}

fn print_message_withot_date(m: &str) {
    println!("{}", m);
}
