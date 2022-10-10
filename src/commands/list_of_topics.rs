use colored::Color;
use kafka::client::KafkaClient;

use crate::print_formated_message;

use super::*;

#[derive(Debug, Args)]
pub struct ListOfTopics {
    ///List of kafka hosts
    #[clap(
        short = 'a',
        long = "a",
        value_parser,
        required = true,
        num_args = 1..,
        help = "List of kafka hosts"
    )]
    host: Vec<String>,

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
        help = "Highlight matched text"
    )]
    color: Option<Vec<u8>>
}

impl ListOfTopics {
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

impl CommandExecute for ListOfTopics {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = KafkaClient::new(self.host.clone());

        client.load_metadata_all()?;

        let topics = client.topics();

        let regex = self.regex();
        for topic in topics {
            if self.regex().is_some() {
                print_formated_message!(
                    topic.name().to_string(),
                    regex.as_ref().unwrap(),
                    self.skip,
                    self.color(),
                    Some(DATE_FORMAT)
                );
            } else {
                print_formated_message!(
                    topic.name().to_string(),
                    None,
                    None
                );
            }
        }

        Ok(())
    }
}
