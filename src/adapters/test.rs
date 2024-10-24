use super::common::Adapter;

#[derive(Default)]
pub struct TestAdapter {
    iter: i32,
}

impl Adapter for TestAdapter {

    fn get_lines(&mut self) -> Option<Vec<String>> {
        self.iter = self.iter + 1;
        Some(vec![format!("{}", self.iter)])
    }
}

