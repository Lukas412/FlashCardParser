pub struct OwnedTopic {
    title: String,
    cards: Vec<OwnedCard>,
}

impl OwnedTopic {
    pub fn new(title: String, cards: Vec<OwnedCard>) -> Self {
        Self { title, cards }
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }

    pub fn cards(&self) -> impl Iterator<Item = &OwnedCard> {
        self.cards.iter()
    }

    pub fn cards_mut(&mut self) -> &mut Vec<OwnedCard> {
        &mut self.cards
    }
}

pub struct OwnedCard {
    question: String,
    answer: String,
}

impl OwnedCard {
    pub fn new(question: String, answer: String) -> Self {
        Self { question, answer }
    }

    pub fn question(&self) -> &str {
        self.question.as_str()
    }

    pub fn question_mut(&mut self) -> &mut String {
        &mut self.question
    }

    pub fn answer(&self) -> &str {
        self.answer.as_str()
    }

    pub fn answer_mut(&mut self) -> &mut String {
        &mut self.answer
    }
}
