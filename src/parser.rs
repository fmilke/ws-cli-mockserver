pub struct Parser;

pub enum ParseResult {
    Send(String),
    List,
    Exit,
    Malformed(String),
}

impl Parser {
    pub fn parse(s: String)  -> ParseResult {
        let s = s.trim();
        let cmd: &str;
        let rest: &str;
        if let Some((c, r)) = s.split_once(' ') {
            cmd = c;
            rest = r;
        } else {
            cmd = s;
            rest = "";
        }

        match cmd {
            "ls" => ParseResult::List,
            "exit" => ParseResult::Exit,
            "send" => ParseResult::Send(String::from(rest)),
            _ => ParseResult::Malformed(format!("could not parse {}", s)),
        }
    }
}
