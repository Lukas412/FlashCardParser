use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::parse::{topic, ParseError};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Topic<'a> {
    title: &'a str,
    cards: Vec<Card<'a>>,
}

impl<'a> Topic<'a> {
    pub fn from_str(input: &'a str) -> Result<Self, ParseError> {
        topic(input)
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

impl<'a> Display for Topic<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Topic: {}", self.title)?;
        writeln!(f, "")?;
        for card in &self.cards {
            writeln!(f, "{}", card)?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
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

impl<'a> Display for Card<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Card ===")?;
        writeln!(f, "Question: {}", self.question)?;
        writeln!(f, "Answer: {}", self.answer)
    }
}
