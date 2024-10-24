use pancurses::Window;

#[derive(Debug)]
pub struct UI {
    pub lines: Vec<String>,
    dirty: bool,
    scroll_pos: u32,
    scroll_locked: bool,
    win: Window,
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

    pub fn move_up(&mut self) {
        self.scroll_pos = self.scroll_pos.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        self.scroll_pos = self.scroll_pos.saturating_add(1);
    }

    pub fn move_to_end(&mut self) {
        self.scroll_pos = 0;
        self.scroll_locked = true;
    }

    pub fn move_to_start(&mut self) {
        self.scroll_locked = false;
    }

    pub fn render(&mut self) {

        if !self.dirty {
            return
        }

        self.win.clear();

        let (max_y, _) = self.win.get_max_yx();
        let total_lines = self.lines.len();
        let mut i = self.lines.len().saturating_sub(max_y as usize);

        while i < total_lines {
            let l = self.lines.get(i)
                .expect("get next line");

            // TODO: handle return value
            self.win.addstr(l);
            self.win.addch('\n' as u32);

            i = i + 1;
        }

        self.win.refresh();
        self.dirty = false;
    }

    pub fn should_exit(&self) -> bool {
        false
    }

    pub fn handle_keyboard(&mut self) {
        match self.win.getch() {
            Some(pancurses::Input::Character('g')) => {
                self.move_to_start();
            },
            Some(pancurses::Input::Character('G')) => {
                self.move_to_end();
            },
            _ => {},
        }
    }
}

