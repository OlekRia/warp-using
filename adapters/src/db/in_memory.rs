use domain::{Question, QuestionId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../../questions.json");
        serde_json::from_str(file).expect("I cannot read questions.json")
    }

    pub fn new() -> Self {
        Self {
            questions: Self::init(),
        }
    }
}

// impl Storable for Store {}

// pub struct InMemory {}
