pub mod question {
    use crate::domain::models::{Question, QuestoinId};

    pub fn new(
        id: QuestoinId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Question {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
