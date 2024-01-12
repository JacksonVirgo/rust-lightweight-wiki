use std::{fs, fmt};

#[derive(Debug)]
pub enum FileError {
    FileNotFoundError,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::FileNotFoundError => write!(f, "File not found"),
        }
    }
}

pub fn load_file(path: &str) -> Result<String, FileError> {
    match fs::read_to_string(path) {
        Ok(file_data) => Ok(file_data),
        Err(_) => Err(FileError::FileNotFoundError)
    }
}