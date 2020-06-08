#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
    SerdeError(serde_json::error::Error),
}
impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::IOError(error)
    }
}
impl From<serde_json::error::Error> for ParseError {
    fn from(error: serde_json::error::Error) -> Self {
        ParseError::SerdeError(error)
    }
}
