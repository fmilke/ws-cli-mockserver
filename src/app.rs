use core::time;
use std::thread;

use crate::ui::UI;
use crate::adapters::common::Adapter;

#[derive(Default)]
pub struct App
{
    ui: UI,
    adapters: Vec<Box::<dyn Adapter>>,
}

impl App {

    pub fn add(&mut self, adapter: Box<dyn Adapter>) {
        self.adapters.push(adapter);
    }

    pub fn run(mut self) {
        self.ui.setup();

        loop {
            eprintln!("looping...");
            self.poll_adapters();
            self.poll_ui();
            if self.ui.should_exit() {
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
                .expect("Adapter to exist");

            if let Err(e) = a.status() {
                // remove with bad status
                self.ui.add_error(e);
                self.adapters.swap_remove(i);
                len = len - 1;
            } else {

                if let Some(lines) = a.get_lines() {
                    eprintln!("got some lines");
                    self.ui.add_lines(lines);
                }

                i = i + 1;
            }

        }
    }

    fn poll_ui(&mut self) {
    }
}
