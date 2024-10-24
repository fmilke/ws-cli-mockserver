use std::{ffi::OsString, fs, path::Path};

use anyhow::anyhow;

#[derive(Clone, Default, Debug)]
pub struct Mocks {
    root: String,
}

impl Mocks {
    pub fn list(&self) -> anyhow::Result<Vec<OsString>> {
        eprintln!("listing messages in {}", self.root);
        let paths = fs::read_dir(&self.root)?;
        let mut r = vec![];
        for p in paths {
            if let Ok(e) = p {
                r.push(e.file_name());
            }
        }

        Ok(r)
    }

    pub fn fetch(&self, path: impl AsRef<Path>) -> anyhow::Result<String> {
        let full = Path::new(self.root.as_str()).join(path);
        let d = full.to_string_lossy().into_owned();
        eprintln!("trying to read message at {}: ", &d);
        match fs::read_to_string(full) {
            Ok(r) => Ok(r),
            Err(e) => {
                eprintln!("failed to read message at {}. reason: {}", d, e);
                Err(anyhow!("could not read message"))
            },
        }
    }

    pub fn from_root(path: String) -> Self {
        Mocks { root: path }
    }
}
