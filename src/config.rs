use std::fs::OpenOptions;
use std::io::{Write};
use serde::{Serialize};

pub trait ReadConfig<T> {
    fn read(s: &str) -> Result<T, serde_yaml::Error>;
}

pub fn generate_struct_yml<T>(filename: &str, data: &T) -> Result<(), Box<dyn std::error::Error>>
    where T: Serialize
{
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&filename)?;

    let s = serde_yaml::to_string(&data)?;
    if s.starts_with("---\n") {
        let re = regex::Regex::new(r"^---\n").unwrap();
        let result = re.replace(&s, "");
        file.write(result.as_bytes())?;
    } else {
        file.write(s.as_bytes())?;
    }

    Ok(())
}
