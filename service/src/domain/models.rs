pub struct QuestoinId(String);

pub struct Question {
    pub id: QuestoinId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
