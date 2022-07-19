use chrono::{DateTime, Local, TimeZone};
use clap::Parser;
use colored::Colorize;
use kafka::{client::KafkaClient, consumer::Consumer};
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        short = 'h',
        long = "host",
        value_parser,
        required = true,
        multiple_values = true
    )]
    host: Vec<String>,

    #[clap(
        short = 't',
        long = "topic",
        value_parser,
        required = true,
        multiple_values = true
    )]
    topic: Vec<String>,

    #[clap(short = 'r', long = "regex", value_parser, required = false)]
    regex: Option<String>,
}

impl Args {
    pub fn consumer(&self) -> Result<Consumer, kafka::Error> {
        let mut builder = Consumer::from_hosts(self.host.clone());

        for topic in self.topic.iter() {
            builder = builder.with_topic(topic.clone());
        }

        builder.create()
    }

    pub fn regex(&self) -> Option<Regex> {
        if let Some(regex) = &self.regex {
            Some(Regex::new(&regex).expect("Wrong regular expression"))
        } else {
            None
        }
    }
}
