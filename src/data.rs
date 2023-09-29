use crate::parse::{topic, ParseError};

pub struct Topic<'a> {
    title: &'a str,
    cards: Vec<Card<'a>>,
}

impl<'a> Topic<'a> {
    pub fn from_str(input: &'a str) -> Result<Self, ParseError> {
        let (_, topic) = topic(input)?;
        Ok(topic)
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card<'a>> {
        self.cards.iter()
    }

    pub(crate) fn new(title: &'a str, cards: Vec<Card<'a>>) -> Self {
        Self { title, cards }
    }
}

pub struct Card<'a> {
    question: &'a str,
    answer: &'a str,
}

impl<'a> Card<'a> {
    pub fn question(&self) -> &str {
        self.question
    }

    pub fn answer(&self) -> &str {
        self.answer
    }

    pub(crate) fn new(question: &'a str, answer: &'a str) -> Self {
        Self { question, answer }
    }
}
