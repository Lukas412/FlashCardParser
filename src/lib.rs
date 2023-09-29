use derive_more::{Display, Error, From};

use crate::parse::ExpectedTopicTitle;

mod parse;

pub fn parse_topic_from_str<'a>(input: &'a str) -> Result<Topic, TopicParseError<'a>> {
    let (input, title) = topic_title(input)?;
    let (input, cards) = many_cards(input)?;
    if input.trim() != "" {
        return Err(TopicParseError::ExpectedToParseToTheEnd { remaining: input });
    }
    Ok(Topic { title, cards })
}

#[derive(Debug, Display, From, Error)]
pub enum TopicParseError<'a> {
    Title(ExpectedTopicTitle<'a>),
    ExpectedToParseToTheEnd { remaining: &'a str },
}

pub struct Topic {
    title: Box<str>,
    cards: Vec<Card>,
}

pub struct Card {
    question: Box<str>,
    answer: Box<str>,
}
