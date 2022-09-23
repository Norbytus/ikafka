use std::ops::{RangeBounds, Range};

use chrono::Local;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

mod args;

const DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_hello_message();

    let args = args::Args::parse();

    let mut consumer = args.consumer()?;

    let regex = args.regex();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let mut m = String::from_utf8(m.value.to_vec()).expect("Wrong value message");
                if regex.is_some() {
                    print_matched_text(regex.as_ref().unwrap(), &mut m);
                } else {
                    print_message(&m);
                }
            }
        }
    }
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

fn print_hello_message() {
    println!("{} {} {}", "Stand".blue(), "with".yellow(), "Satan".red());
}
