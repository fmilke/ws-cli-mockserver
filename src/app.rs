use core::time;
use std::path::Path;
use std::{fs, thread};

use crate::parser::{ParseResult, Parser};
use crate::ui::UI;
use crate::adapters::common::{Adapter, Direction, Line};

#[derive(Default)]
pub struct App
{
    ui: UI,
    adapters: Vec<Box::<dyn Adapter>>,
    should_exit: bool,
}

impl App {

    pub fn add(&mut self, adapter: Box<dyn Adapter>) {
        self.adapters.push(adapter);
    }

    pub fn run(mut self) {
        self.ui.setup();

        loop {
            self.poll_adapters();
            self.poll_keyboard();
            if self.should_exit {
                break
            }

            self.ui.render();

            thread::sleep(time::Duration::from_millis(30));
        }

        self.ui.teardown();
    }

    fn poll_adapters(&mut self) {
        let mut len = self.adapters.len();
        let mut i = 0;

        while i < len {
            let a = self.adapters.get_mut(i)
                .expect("adapter to exist");

            if let Err(e) = a.status() {
                // remove with bad status
                self.ui.add_error(e);
                self.adapters.swap_remove(i);
                len = len - 1;
            } else {

                if let Some(lines) = a.get_lines() {
                    self.ui.add_lines(lines);
                }

                i = i + 1;
            }

        }
    }

    fn poll_keyboard(&mut self) {
        if let Some(command) = self.ui.handle_keyboard() {
            match Parser::parse(command) {
                ParseResult::Exit => {
                    self.should_exit = true;
                },
                ParseResult::Help => {
                    self.ui.print_help();
                },
                ParseResult::List => {
                    eprintln!("listing items");
                    self.list_items();
                },
                ParseResult::Send(list) => {
                    eprintln!("sending items: {}", list);
                    self.send_message(list);
                },
                ParseResult::Malformed(s) => {
                    eprintln!("malformed command: {}", s);
                },
            }
        }
    }

    fn list_items(&mut self) {
        let p = Path::new("./mocks");
        match fs::read_dir(p) {
            Err(e) => {
                eprintln!("could not list files: {}", e);
                self.ui.add_line(Line::new_log(String::from("cannot list files. failed to read directory.")));
            },
            Ok(dir) => {
                for p in dir {
                    match  p {
                        Ok(f) => self.ui.add_line(Line::new_log(f.file_name().to_string_lossy().to_string())),
                        Err(e) => eprintln!("could not read file: {}", e),
                    }
                }
            },
        }
    }

    fn send_message(&mut self, file_name: String) {
        let file_name = String::from("mocks/") + file_name.as_str();

        match fs::read_to_string(&file_name) {
            Ok(content) => {
                self.ui.add_line(Line::new_json(content.clone(), Direction::Outgoing));

                for a in self.adapters.iter_mut() {
                    a.send_message(&content);
                }
            },
            Err(e) => {
                self.ui.add_line(Line::new_log(format!("could not read file: {}", file_name)));
                eprintln!("could not read file {}: {}", file_name, e);
            },
        }
    }
}
