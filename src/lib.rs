mod parse;

pub struct Topic {
    title: Box<str>,
    cards: Vec<Card>,
}

pub struct Card {
    question: Box<str>,
    answer: Box<str>,
}
