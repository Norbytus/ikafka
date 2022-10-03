use super::*;
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
}

impl CommandExecute for ReadMessages {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut consumer = self.consumer()?;

        let regex = self.regex();

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
}


