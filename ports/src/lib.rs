use domain::{Question, QuestionId};
use std::collections::HashMap;

pub trait Storable: Send + Sync {
    fn init() -> Box<dyn Storable>
    where
        Self: Sized;

    fn get_questions(&self) -> &HashMap<QuestionId, Question>;

    fn clone_box(&self) -> Box<dyn Storable>;
}

impl Clone for Box<dyn Storable> {
    fn clone(&self) -> Box<dyn Storable> {
        self.clone_box()
    }
}
