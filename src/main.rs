use chrono::Local;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    let mut consumer = args.consumer()?;

    let regex = args.regex();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let mut m = String::from_utf8(m.value.to_vec()).expect("Wrong value message");
                let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                if regex.is_some() {
                    print_matched_text(regex.as_ref().unwrap(), &mut m);
                } else {
                    println!("[{}] - {}", date, m);
                }
            }
        }
    }
}

fn print_matched_text(regex: &Regex, m: &mut String) {
    for mat in regex.find_iter(m.clone().as_str()).into_iter() {
        let word = &m[mat.start()..mat.end()];

        m.replace_range(mat.start()..mat.end(), word.red().to_string().as_str());
    }

    println!("{}", m);
}
