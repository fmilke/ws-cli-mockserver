use anyhow::Result;

pub trait Adapter {

    fn status(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_lines(&mut self) -> Option<Vec<String>> {
        None
    }
}

