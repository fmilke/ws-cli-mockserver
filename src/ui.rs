use pancurses::Window;

use crate::{adapters::common::Line, parser::HELP_TEXT};

const CHAR_DEL: char = 0x7F as char;
const CHAR_ESC: char = 27 as char;
const CHAR_EOL: char = 10 as char;

#[derive(Debug)]
pub struct UI {
    pub lines: Vec<Line>,
    dirty: bool,
    scroll_pos: u32,
    scroll_locked: bool,
    win: Window,
    cli: CommandLine,
}

impl Default for UI {
    fn default() -> Self {
        UI::new()
    }
}

impl UI {
    pub fn new() -> Self {
        UI{
            lines: vec![],
            dirty: false,
            scroll_pos: 0,
            scroll_locked: true,
            win: pancurses::initscr(),
            cli: CommandLine::default(),
        }
    }

    pub fn setup(&mut self) {
        // ncurses setup
        // currently done in constructor
        //pancurses::initscr();
        pancurses::noecho();
        pancurses::cbreak();
        self.win.timeout(5);
    }

    pub fn teardown(&mut self) {
        // ncurses teardown
        pancurses::endwin();
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
        self.dirty = true;
    }

    pub fn add_lines(&mut self, lines: Vec<Line>) {
        for l in lines {
            self.add_line(l);
        }
    }

    pub fn add_error(&mut self, e: anyhow::Error) {
        self.add_line(Line::new_log(format!("{}", e)));
    }

    pub fn move_up(&mut self) {

        if self.scroll_locked {
            let max_y = self.win.get_max_y();
            self.scroll_pos = self.lines.len()
                .saturating_sub(max_y as usize)
                .saturating_sub(1) as u32;
            self.scroll_locked = false;
        } else {
            self.scroll_pos = self.scroll_pos.saturating_sub(1);
            self.scroll_locked = false;
        }
    }

    pub fn move_down(&mut self) {
        self.scroll_pos = self.scroll_pos.saturating_add(1).min(self.lines.len() as u32);
    }

    pub fn move_to_end(&mut self) {
        self.scroll_pos = 0;
        self.scroll_locked = true;
    }

    pub fn move_to_start(&mut self) {
        self.scroll_locked = false;
    }

    pub fn main_win_height(&self) -> i32 {
        let (max_y, _) = self.win.get_max_yx();

        // one line for commands
        max_y.saturating_sub(1)
    }

    pub fn render(&mut self) {

        if !self.dirty {
            return
        }

        // TODO: do we have to clear? either way, reduce flickering
        self.win.clear();

        self.render_main_win();
        self.render_command_line();

        self.win.refresh();
        self.dirty = false;
    }

    pub fn render_main_win(&mut self) {
        let max_y = self.main_win_height();
        let mut total_lines = self.lines.len();

        // by default show last lineself.u
        let mut i = self.lines.len().saturating_sub(max_y as usize);

        // but when scrolling, show lines after scroll_pos
        if !self.scroll_locked {
            i = self.scroll_pos as usize;
            total_lines = total_lines.min(i + max_y as usize);
        }

        while i < total_lines {
            let l = self.lines.get(i)
                .expect("get next line");

            self.render_line(l);

            i = i + 1;
        }
    }

    fn render_line(&self, l: &Line) {

        if l.treat_as_json {
            if l.outgoing() {
                self.win.addch('-');
                self.win.addch('>');
            } else {
                self.win.addch('<');
                self.win.addch('-');
            }

            self.win.addch(' ');
        } else {
            self.win.addch(' ');
            self.win.addch(' ');
            self.win.addch(' ');
        }

        // TODO: handle return value
        self.win.addstr(l.text.as_str());
        self.win.addch('\n' as u32);
    }

    pub fn render_command_line(&self) {
        if !self.cli.has_focus {
            return;
        }

        let (max_y, _) = self.win.get_max_yx();
        let y = max_y.saturating_sub(1);

        self.win.mvprintw(y, 0, &self.cli.command);
    }

    pub fn handle_keyboard(&mut self) -> Option<String> {
        match self.win.getch() {
            Some(pancurses::Input::KeyExit) => {
                self.cli.exit();
                self.dirty = true;
            },
            Some(pancurses::Input::KeyBackspace) => {
                self.cli.backspace();
                self.dirty = true;
            },
            Some(pancurses::Input::Character(c)) => {
                self.dirty = true;
                if self.cli.has_focus {
                    eprintln!("char is {}, {}", c, c as u32);
                    match c {
                        CHAR_DEL => {
                            self.cli.backspace();
                        },
                        CHAR_ESC => {
                            self.cli.exit();
                        },
                        CHAR_EOL => {
                            return self.cli.complete();
                        },
                        _ => {
                            self.cli.push_char(c);
                        },
                    }
                } else {
                    match c {
                        ':' => {
                            self.cli.focus();
                        },
                        'g' => {
                            self.move_to_start();
                        },
                        'G' => {
                            self.move_to_end();
                        },
                        'j' => {
                            self.move_down();
                        },
                        'k' => {
                            self.move_up();
                        },
                        _ => {},
                    }
                }
            },
            // TODO: use {} to move by screen height
            _ => {},
        }
        
        None
    }

    pub fn print_help(&mut self) {
        for l in HELP_TEXT.lines() {
            self.add_line(Line::new_log(String::from(l)));
        }
    }
}


#[derive(Default, Debug)]
struct CommandLine {
    has_focus: bool,
    command: String,
}

impl CommandLine {
    pub fn push_char(&mut self, c: char) {
        self.command.push(c);
    }

    pub fn blur(&mut self) {
        self.has_focus = false;
    }

    pub fn focus(&mut self) {
        self.has_focus = true;
        self.command = String::from(":")
    }

    pub fn backspace(&mut self) {
        if !self.has_focus {
            return
        }

        if self.command.len() > 1 {
            self.command.pop();
        } else {
            self.blur();
        }
    }

    pub fn exit(&mut self) {
        self.command = String::new();
        self.blur();
    }

    pub fn complete(&mut self) -> Option<String> {
        if self.command.len() > 0 {
            let mut command = String::new();
            std::mem::swap(&mut command, &mut self.command);
            self.blur();
            return Some(command);
        }

        None
    }
}
