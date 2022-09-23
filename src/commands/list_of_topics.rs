use kafka::client::KafkaClient;

use super::*;

#[derive(Debug, Args)]
pub struct ListOfTopics {
    ///List of kafka hosts
    #[clap(
        short = 'a',
        long = "a",
        value_parser,
        required = true,
        multiple_values = true,
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
}

impl ListOfTopics {
    pub fn regex(&self) -> Option<Regex> {
        if let Some(regex) = &self.regex {
            Some(Regex::new(&regex).expect("Wrong regular expression"))
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

        for topic in topics {
            if self.regex().is_some() {
                print_matched_text_witout_date(
                    self.regex().as_ref().unwrap(),
                    &mut topic.name().to_string()
                );
            } else {
                print_message_withot_date(topic.name());
            }
        }

        Ok(())
    }
}
