use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageResource {
    pub key: Option<String>,
    pub message: String,
}
impl Display for MessageResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MessageResource Key: {:#?}, Message: {}", self.key, self.message)
    }
}
impl MessageResource {
    pub fn new(key: &str, msg: &str) -> Self {
        Self { key: Some(key.to_string()), message: msg.to_string() }
    }
    pub fn new_from_str(msg: &str) -> Self {
        Self { key: None, message: msg.to_string() }
    }
    pub fn new_from_string(msg: String) -> Self {
        Self { key: None, message: msg }
    }
}
impl<T> From<T> for MessageResource 
where T: std::error::Error
{
    fn from(err: T) -> Self {
        Self { key: None, message: err.to_string() }
    }
}
impl Default for MessageResource{
    fn default() -> Self {
        Self { key: None, message: "".to_string() }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Network(MessageResource),
    IO(MessageResource),
    Privilege(MessageResource),
    UnexpectedStatusCode(u16, u16, Vec<MessageResource>),
    Serde(MessageResource),
    Parser(MessageResource),
    Unspecified
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Network(message) => write!(f, "Error of type Network. MessageResource: {message}"),
            Error::IO(message) => write!(f, "Error of type IO. MessageResource: {message}"),
            Error::Privilege(message) => write!(f, "Error of type Privilege. MessageResource: {message}"),
            Error::UnexpectedStatusCode(expected, actual, messages) => write!(f, "Error of type UnexpectedStatusCode. Expected: {expected}, Actual: {actual}, MessageResources: {:#?}", messages),
            Error::Serde(message) => write!(f, "Error of type Serialization/Deserialization. MessageResource: {message}"),
            Error::Unspecified => write!(f, "Error of type Unspecified."),
            Error::Parser(message) => write!(f, "Error of type Parser. MessageResource: {message}"),
        }
    }
}
impl std::error::Error for Error {}