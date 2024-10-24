use std::result::Result;

#[derive(Debug, Clone)]
pub enum SplitError {
    Tainted,
}

#[derive(Debug, Clone, Copy)]
enum ParseMode {
    ObjectOrArray,
    StringLiteral,
    RootLevelValue,
    Undetermined,
}

#[derive(Debug, Clone)]
pub struct JsonSplitter {
    max_size: usize,
    buffer: Vec<u8>,
    depth: u32,
    mode: ParseMode,
    escaping_next_char: bool,
    tainted: bool,
}

const SYM_QUOTES: char = '"';
const SYM_OPEN_CURLY: char = '{';
const SYM_CLOSE_CURLY: char = '}';
const SYM_OPEN_BRACKET: char = '[';
const SYM_CLOSE_BRACKET: char = ']';
const SYM_BACKSLASH: char = '\\';

const ROOT_LEVEL_CHARS: [char; 22] = [
    't',
    'r',
    'u',
    'e',
    'E',
    'f',
    'a',
    'l',
    's',
    'n',
    '-',
    '.',
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
];

impl JsonSplitter {
    pub fn new(max_size: usize) -> Self {
        JsonSplitter{
            max_size,
            buffer: vec![],
            depth: 0,
            mode: ParseMode::Undetermined,
            escaping_next_char: false,
            tainted: false,
        }
    }

    pub fn push(&mut self, buffer: &[u8]) -> Result<Vec<String>, SplitError> {

        if self.tainted {
            return Err(SplitError::Tainted);
        }

        let mut r = vec![];
        let mut i = 0;
        let mut start = 0;
        while i < buffer.len() {

            // todo: properly grab chars
            // depending on encoding
            let c = buffer[i] as char;

            match self.mode {
                ParseMode::Undetermined => {
                    match c {
                        SYM_OPEN_CURLY | SYM_OPEN_BRACKET => {
                            self.mode = ParseMode::ObjectOrArray;
                            self.depth = self.depth + 1;
                            start = i;
                        },
                        SYM_CLOSE_CURLY | SYM_CLOSE_BRACKET => {
                            self.tainted = true;
                            return Err(SplitError::Tainted);
                        },
                        SYM_QUOTES => {
                            self.mode = ParseMode::StringLiteral;
                            self.depth = self.depth + 1;
                        },
                        _ => {
                            if !c.is_whitespace() {
                                self.mode = ParseMode::RootLevelValue;
                                start = i;
                            }
                        },
                    }
                },
                ParseMode::StringLiteral => {
                    match (c, self.escaping_next_char) {
                        (SYM_QUOTES, true) => {
                            // ignore escaped quotes
                            if self.escaping_next_char {
                                self.escaping_next_char = false;
                            }
                        },
                        (SYM_QUOTES, false) => {
                            // we have as single string 
                            // as complete json payload
                            // therefore, extract the message
                            if self.depth == 1 {
                                let s = String::from_utf8_lossy(&buffer[start..i+1]);
                                r.push(s.to_string());
                                start = i + 1;
                                self.depth = 0;
                                self.mode = ParseMode::Undetermined;
                            } else {
                                self.depth = self.depth - 1;
                                self.mode = ParseMode::ObjectOrArray;
                            }
                        },
                        (SYM_BACKSLASH,_) => {
                            self.escaping_next_char = !self.escaping_next_char;
                        },
                        _ => {},
                    }
                },
                ParseMode::RootLevelValue => {
                    if !JsonSplitter::is_root_level_char(c) {
                        let s = String::from_utf8_lossy(&buffer[start..i+1]);
                        r.push(s.to_string());
                        start = i + 1;
                    }
                },
                ParseMode::ObjectOrArray => {
                    match c {
                        SYM_QUOTES => {
                            self.mode = ParseMode::StringLiteral;
                            self.depth = self.depth + 1;
                        },
                        SYM_OPEN_CURLY | SYM_OPEN_BRACKET => {
                            self.depth = self.depth + 1;
                        },
                        SYM_CLOSE_BRACKET | SYM_CLOSE_CURLY => {
                            if self.depth == 0 {
                                // we had closing bracket, but no openening bracket
                                self.tainted = true;
                                return Err(SplitError::Tainted);
                            } else if self.depth == 1 {
                                let s = String::from_utf8_lossy(&buffer[start..i+1]);
                                r.push(s.to_string());
                                start = i + 1;
                                self.depth = 0;
                                self.mode = ParseMode::Undetermined;
                            } else {
                                self.depth = self.depth - 1 ;
                            }
                        },
                        _ => {},
                    }
                },
            }
            i = i + 1;
        }

        if start < buffer.len() {

        }

        Ok(r)
    }

    fn flush(&mut self) -> Result<Vec<String>, SplitError> {
        let r = vec![];

        if self.tainted {
            return Err(SplitError::Tainted);
        }

        match self.mode {
            ParseMode::Undetermined => {},
            ParseMode::ObjectOrArray => {},
            ParseMode::RootLevelValue => {
            },
        }

        Ok(r)
    }

    fn push_and_flush(&mut self, buffer: &[u8]) -> Result<Vec<String>, SplitError> {
        match self.push(buffer) {
            Ok(mut s) => {
                match self.flush() {
                    Ok(ref mut s2) => {
                        s.append(s2);
                        Ok(s)
                    },
                    e => e,
                }
            },
            e => e,
        }
    }

    fn is_root_level_char(c: char) -> bool {
        ROOT_LEVEL_CHARS.contains(&c)
    }
}

impl Default for JsonSplitter {
    fn default() -> Self {
        JsonSplitter::new(4_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_string_literal() {
        let mut splitter = JsonSplitter::default();
        let a = "\"prop\"";
        let messages  = splitter.push(a.as_bytes()).unwrap();
        assert!(messages.len() > 0);
        assert_eq!(messages[0], a);
    }

    #[test]
    fn parse_two_string_literals() {
        let mut splitter = JsonSplitter::default();
        let a = "\"prop\"\"second\"";
        let messages  = splitter.push(a.as_bytes()).unwrap();
        assert!(messages.len() == 2);
        assert_eq!(messages[0], "\"prop\"");
        assert_eq!(messages[1], "\"second\"");
    }

    #[test]
    fn split_two_strings_with_one_having_escaped_delimiter() {
        let mut splitter = JsonSplitter::default();
        let a = "\"first\\\" part\"\"second\"";
        let messages  = splitter.push(a.as_bytes()).unwrap();
        assert!(messages.len() == 2);
        assert_eq!(messages[0], "\"first\\\" part\"");
        assert_eq!(messages[1], "\"second\"");
    }

    #[test]
    fn should_taint_on_bracket_on_not_produce_any_messages() {
        let invalid = "}";
        let valid = "\"first\"";

        let mut splitter = JsonSplitter::default();
        let messages = splitter.push(valid.as_bytes()).unwrap();
        assert!(messages.len() == 1);

        let mut splitter = JsonSplitter::default();
        let _  = splitter.push(valid.as_bytes());
        let messages = splitter.push(invalid.as_bytes());
        assert!(messages.is_err());
    }

    #[test]
    fn parse_simple_message() {
        let mut splitter = JsonSplitter::default();
        let a = "{\"prop\": 1}";
        let messages  = splitter.push(a.as_bytes()).unwrap();
        assert!(messages.len() > 0);
        assert_eq!(messages[0], a);
    }

    #[test]
    fn parse_null() {
        let mut splitter = JsonSplitter::default();
        let a = "null";
        let messages  = splitter.push_and_flush(a.as_bytes()).unwrap();
        assert!(messages.len() > 0);
        assert_eq!(messages[0], a);
    }
}
