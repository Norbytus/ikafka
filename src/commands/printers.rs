use colored::Color;
use super::*;

pub trait Formatter {
    fn formate(&mut self) -> Option<&str>;
}

pub struct DefaultFormatter<'a> {
    text: String,
    color: Option<Color>,
    date: Option<&'a str>
}

impl<'a> DefaultFormatter<'a> {
    pub fn new(text: String, color: Option<Color>, date: Option<&'a str>) -> Self {
        Self { text, color, date }
    }
}

impl<'a> Formatter for DefaultFormatter<'a> {
    fn formate(&mut self) -> Option<&str> {
        if let Some(date) = self.date.as_ref() {
            let data = format!("[{}] ", Local::now().format(date).to_string().as_str());
            self.text.insert_str(0, data.as_str());
        }

        if let Some(color) = self.color {
            self.text = self.text.color(color).to_string();
        }

        Some(self.text.as_ref())
    }
}

pub struct RegexFormatter<'a> {
    regex: &'a Regex,
    skip: bool,
    text: String,
    color: Option<Color>,
    date: Option<&'a str>
}

impl<'a> RegexFormatter<'a> {
    pub fn new(
        text: String,
        regex: &'a Regex,
        skip: bool,
        color: Option<Color>,
        date: Option<&'a str>
    ) -> Self {
        Self {
            regex,
            skip,
            text,
            color,
            date
        }
    }
}

impl<'a> Formatter for RegexFormatter<'a> {
    fn formate(&mut self) -> Option<&str> {
        let matches: Vec<Range<usize>> = self
            .regex
            .find_iter(self.text.as_str())
            .into_iter()
            .map(|mat| mat.start()..mat.end())
            .collect();

        if self.skip && matches.is_empty() {
            return None;
        }

        for mat in matches {
            let word = &self.text[mat.clone()];

            self.text
                .replace_range(mat, word.red().to_string().as_str());
        }

        if let Some(date) = self.date.as_ref() {
            let data = format!("[{}] ", Local::now().format(date).to_string().as_str());
            self.text.insert_str(0, data.as_str());
        }

        Some(self.text.as_ref())
    }
}

#[macro_export]
macro_rules! print_formated_message {
    ($text:expr, $date:expr, $color:expr) => {
        let mut f = crate::commands::printers::DefaultFormatter::new(
            $text,
            $date,
            $color,
        );
        println!("{}", crate::commands::printers::Formatter::formate(&mut f).unwrap());
    };
    ($text:expr, $regex:expr, $skip:expr, $color:expr, $date:expr) => {
        let mut s = crate::commands::printers::RegexFormatter::new(
            $text,
            $regex,
            $skip,
            $color,
            $date
        );
        let f = crate::commands::printers::Formatter::formate(&mut s);

        if let Some(d) = f {
            println!("{}", d);
        }
    };
}
