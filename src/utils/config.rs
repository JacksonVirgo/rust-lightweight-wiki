use std::{fmt, env};
use dotenv::dotenv;

#[derive(Debug)]
pub enum AppError {
    EnvVarNotFound,
    ParseError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::EnvVarNotFound => write!(f, "Environment variable not found"),
            AppError::ParseError => write!(f, "Error parsing value"),
        }
    }
}

pub fn get_host()-> Result<String, AppError> {
    dotenv().ok();
    match env::var("HOST") {
        Ok(str_value) => Ok(str_value),
        Err(_) => Err(AppError::EnvVarNotFound)
    }
}

pub fn get_port() -> Result<u16, AppError> {
    match env::var("PORT") {
        Ok(str_value) => {
            str_value
                .parse::<u16>()
                .map_err(|_| AppError::ParseError)
        }
        Err(_) => Err(AppError::EnvVarNotFound),
    }
}
