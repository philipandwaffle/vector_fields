use std::{error::Error, fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};

pub struct JSONParser;
impl JSONParser {
    pub fn load<T: for<'a> Deserialize<'a> + Serialize>(path: &str) -> Result<T, Box<dyn Error>> {
        let json_file_path = Path::new(path);
        let file = File::open(json_file_path)?;
        let json: T = serde_json::from_reader(file)?;
        Ok(json)
    }

    pub fn save<T: for<'a> Deserialize<'a> + Serialize>(
        path: &str,
        value: &T,
    ) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        serde_json::to_writer_pretty(file.by_ref(), value)?;
        file.flush()?;
        Ok(())
    }
}
