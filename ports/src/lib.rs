use domain::{Question, QuestionId};
use std::collections::HashMap;

pub trait Storable: Send + Sync {
    fn init() -> Box<dyn Storable>
    where
        Self: Sized;
    fn get_questions(&self) -> &HashMap<QuestionId, Question>;
}
