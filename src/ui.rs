#[derive(Debug, Default)]
pub struct UI {
    pub lines: Vec<String>,
    dirty: bool,
    offset_y: u32,
}

impl UI {
    pub fn new() -> Self {
        UI{
            lines: vec![],
            dirty: false,
            offset_y: 0,
        }
    }

    pub fn setup(&mut self) {
        // ncurses setup
        ncurses::initscr();
        ncurses::noecho();
        ncurses::cbreak();
        ncurses::timeout(5);
    }

    pub fn teardown(&mut self) {
        // ncurses teardown
        ncurses::endwin();
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
        self.dirty = true;
    }

    pub fn add_lines(&mut self, lines: Vec<String>) {
        for l in lines {
            self.add_line(l);
        }
    }

    pub fn add_error(&mut self, e: anyhow::Error) {
        self.add_line(format!("{}", e));
    }

    pub fn render(&mut self) {

        if !self.dirty {
            return
        }

        ncurses::clear();

        let mut max_x = 0;
        let mut max_y = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut max_y, &mut max_x);

        let total_lines = self.lines.len();
        let mut i = self.lines.len().saturating_sub(max_y as usize);

        while i < total_lines {
            let l = self.lines.get(i)
                .expect("get next line");

            match ncurses::addstr(l.as_str()) {
                Ok(_) => {
                    ncurses::addch('\n' as u32);
                },
                Err(e) => {
                    eprintln!("cannot print line: {}", e);
                    ncurses::addch('\n' as u32);
                    return
                },
            }

            i = i + 1;
        }

        ncurses::refresh();
        self.dirty = false;
    }

    pub fn should_exit(&self) -> bool {
        false
    }
}

