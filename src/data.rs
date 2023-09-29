pub struct Topic<'a> {
    title: &'a str,
    cards: Vec<Card<'a>>,
}

impl<'a> Topic<'a> {
    pub(crate) fn new(title: &'a str, cards: Vec<Card<'a>>) -> Self {
        Self { title, cards }
    }
}

pub struct Card<'a> {
    question: &'a str,
    answer: &'a str,
}

impl<'a> Card<'a> {
    pub(crate) fn new(question: &'a str, answer: &'a str) -> Self {
        Self { question, answer }
    }
}
