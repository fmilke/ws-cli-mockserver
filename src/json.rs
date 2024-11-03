use anyhow::anyhow;

#[derive(Debug, Default)]
pub struct JsonFormatter;

impl JsonFormatter {
    pub fn format(&mut self, s: &String) -> anyhow::Result<String> {

        let mut r = String::new();
        let mut inside_string = false;
        let mut escaping = false;

        for c in s.chars() {
            match c {
                '"' => {
                    r.push(c);
                    if escaping {
                        continue;
                    } else {
                        inside_string = !inside_string;
                    }
                },
                '\\' => {
                    r.push(c);
                    if inside_string {
                        escaping = true;
                    } else {
                        return Err(anyhow!("Unexpected '\\'"));
                    }
                },
                c => {
                    if !c.is_control() {
                        if inside_string || !c.is_whitespace() {
                            r.push(c);
                        }
                    }
                },
            }
        }

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_string_literal()  {
        let mut fmt = JsonFormatter::default();
        let inp = String::from("\"This is string\"");
        let out = fmt.format(&inp).unwrap();
        assert_eq!(inp, out);
    }

    #[test]
    fn format_string_literal_preserve_whitespace()  {
        let mut fmt = JsonFormatter::default();
        let inp = String::from("\"This is string\"");
        let out = fmt.format(&inp).unwrap();
        assert_eq!(inp, out);
    }


    #[test]
    fn format_simple_array()  {
        let mut fmt = JsonFormatter::default();
        let inp = String::from("[1,2]");
        let out = fmt.format(&inp).unwrap();
        assert_eq!(inp, out);
    }

    #[test]
    fn format_simple_array_with_line_break()  {
        let mut fmt = JsonFormatter::default();
        let inp = String::from("[1,\n2]");
        let out = fmt.format(&inp).unwrap();
        assert_eq!(String::from("[1,2]"), out);
    }
}

