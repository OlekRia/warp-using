use domain::{Question, QuestionId};

pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Question {
    Question {
        id,
        title,
        content,
        tags,
    }
}
