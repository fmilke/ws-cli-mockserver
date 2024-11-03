use std::time::Instant;
use anyhow::Result;

#[derive(Default, Debug)]
pub enum Direction {
    Outgoing,
    #[default]
    Incoming,
}

use crate::json::JsonFormatter;

#[derive(Debug)]
pub struct Line {
    pub timestamp: Instant,
    pub text: String,
    pub treat_as_json: bool,
    pub invalid_json: bool,
    pub dir: Direction,
}

impl Line {
    pub fn new_json(s: String, d: Direction) -> Self {
        let mut fmt = JsonFormatter::default();
        match fmt.format(&s) {
            Ok(s) =>  {
                Line {
                    timestamp: Instant::now(),
                    treat_as_json: true,
                    text: s,
                    invalid_json: false,
                    dir: d,
                }
            },
            Err(e) => {
                eprintln!("message was not valid json. error: {}. json: {}", e, s);
                let mut line = Line::new_log(s);
                line.invalid_json = true;
                line
            },
        }
    }

    pub fn new_log(s: String) -> Self {
        Line {
            timestamp: Instant::now(),
            treat_as_json: false,
            text: s,
            invalid_json: false,
            dir: Direction::Outgoing,
        }
    }

    pub fn format_date(&self) -> String {
        String::new()
    }

    pub fn outgoing(&self) -> bool {
        match self.dir {
            Direction::Outgoing => {
               true 
            },
            _ => {
                false
            },
        }
    }
}

pub trait Adapter {

    fn status(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_lines(&mut self) -> Option<Vec<Line>> {
        None
    }

    fn send_message(&mut self, _: &String) {
    }
}

