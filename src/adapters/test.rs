use super::common::{Adapter, Line};

#[derive(Default)]
pub struct TestAdapter {
    iter: i32,
}

impl Adapter for TestAdapter {

    fn get_lines(&mut self) -> Option<Vec<Line>> {
        self.iter = self.iter + 1;

        if self.iter % 5 == 0 {
            Some(vec![Line::new_log(format!("{}", self.iter))])
        } else {
            None
        }
    }
}

