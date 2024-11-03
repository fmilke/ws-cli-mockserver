pub struct Parser;

pub const HELP_TEXT: &str = r"
Allowed commands:

:ls                  - List all available json messages
:exit                - End program
:help, :h            - Print help text
:send, :s <file>     - Send json message. <file> must be one of the files listed with :ls
";

pub enum ParseResult {
    Send(String),
    List,
    Help,
    Exit,
    Malformed(String),
}

impl Parser {
    pub fn parse(command: String)  -> ParseResult {
        let mut s = command.trim();
        if let Some((_, r)) = s.split_once(':') {
            s = r;
        } else {
            ParseResult::Malformed(command.clone());
        }

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
            "help" | "h" => ParseResult::Help,
            "send" | "s" => ParseResult::Send(String::from(rest)),
            _ => ParseResult::Malformed(format!("could not parse {}", s)),
        }
    }
}

