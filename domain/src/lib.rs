use serde::{Deserialize, Serialize};
use std::{
    cmp::Eq,
    hash::Hash,
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct QuestionId(pub String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        if id.is_empty() {
            Err(Error::new(ErrorKind::InvalidData, "Id is empty"))
        } else {
            Ok(Self(id.to_owned()))
        }
    }
}

//

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    #[must_use]
    pub const fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}
