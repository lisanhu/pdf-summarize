pub mod google;
pub mod openai;

pub trait Summarize {
    fn summarize(&self, text: &str) -> Result<String, Error> ;
}

#[derive(Debug)]
pub struct Error  {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}
