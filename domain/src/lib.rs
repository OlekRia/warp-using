use serde::Serialize;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug, Serialize)]
pub struct QuestionId(pub String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_owned())),
            true => Err(Error::new(ErrorKind::InvalidData, "Id is empty")),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
