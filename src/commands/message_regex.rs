use crate::print_formated_message;

use super::*;
use colored::Color;
use kafka::consumer::Consumer;

#[derive(Args, Debug)]
pub struct ReadMessages {
    ///List of kafka hosts
    #[clap(
        short = 'a',
        long = "a",
        value_parser,
        required = true,
        num_args = 0..,
        help = "List of kafka hosts"
    )]
    host: Vec<String>,

    ///List of kafka topics
    #[clap(
        short = 't',
        long = "topic",
        value_parser,
        required = true,
        num_args = 0..,
        help = "List of kafka topics"
    )]
    topic: Vec<String>,

    ///Regular expression for message match
    #[clap(
        short = 'r',
        long = "regex",
        value_parser,
        required = false,
        help = "Regular expression for message match"
    )]
    regex: Option<String>,

    ///Skip not suitable messages, true by default
    #[clap(
        short = 's',
        long = "skip",
        required = false,
        requires = "regex",
        help = "Skip not suitable messages, true by default"
    )]
    skip: bool,

    ///Highlight matched text format 0-255 0-255 0-255
    #[clap(
        short = 'c',
        long = "color",
        required = false,
        requires = "regex",
        num_args = 3,
        value_parser,
        help = "Highlight matched text"
    )]
    color: Option<Vec<u8>>
}

impl ReadMessages {
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

    pub fn color(&self) -> Option<Color> {
        if let Some(color) = &self.color {
            Some(Color::TrueColor {
                r: color.get(0).unwrap_or(&0).clone(),
                g: color.get(1).unwrap_or(&0).clone(),
                b: color.get(3).unwrap_or(&0).clone()
            })
        } else {
            None
        }
    }
}

impl CommandExecute for ReadMessages {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut consumer = self.consumer()?;

        let regex = self.regex();

        loop {
            for ms in consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    let m = String::from_utf8(m.value.to_vec()).expect("Wrong value message");
                    if regex.is_some() {
                        print_formated_message!(
                            m,
                            regex.as_ref().unwrap(),
                            self.skip,
                            self.color(),
                            Some(DATE_FORMAT)
                        );
                    } else {
                        print_formated_message!(
                            m,
                            None,
                            Some(DATE_FORMAT)
                        );
                    }
                }
            }
        }
    }
}


