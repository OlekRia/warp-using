use domain::{Question, QuestionId};
use ports::Storable;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Storable for Store {
    fn init() -> Box<dyn Storable> {
        let file = include_str!("../../questions.json");
        let questions: HashMap<QuestionId, Question> =
            serde_json::from_str(file).expect("Failed to read questions.json");
        Box::new(Self { questions })
    }

    fn get_questions(&self) -> &HashMap<QuestionId, Question> {
        &self.questions
    }

    fn clone_box(&self) -> Box<dyn Storable> {
        Box::new(self.clone())
    }
}
