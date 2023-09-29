use crate::{Card, Topic};
use derive_more::{Display, Error, From};
use nom::FindSubstring;

#[derive(Debug, Display, Error, From)]
pub enum ParseError<'a> {
    TopicTitleIsEmpty,
    TopicTitleIsMultipleLinesLong { title: &'a str },
    CardQuestionIsEmpty,
    CardAnswerIsIsEmpty,
}

pub(crate) fn topic<'a>(input: &'a str) -> Result<(&'a str, Topic<'a>), ParseError> {
    let input = input.trim();
    let (input, title) = topic_title(input)?;
    let cards = cards(input)?;
    Ok((input, Topic::new(title, cards)))
}

fn topic_title(input: &str) -> Result<(&str, &str), ParseError> {
    let (input, text) =
        text_until_card_separator(input).map_err(|_| ParseError::TopicTitleIsEmpty)?;
    if text.contains("\n") {
        return Err(ParseError::TopicTitleIsMultipleLinesLong { title: text });
    }
    Ok((input, text))
}

fn cards(mut input: &str) -> Result<Vec<Card>, ParseError> {
    let mut results = Vec::new();
    while !input.is_empty() {
        let (remaining, card) = card(input)?;
        results.push(card);
        input = remaining;
    }
    Ok(results)
}

fn card(input: &str) -> Result<(&str, Card), ParseError> {
    let (input, question) =
        text_until_card_divider(input).map_err(|_| ParseError::CardQuestionIsEmpty)?;
    let (input, answer) =
        text_until_card_separator(input).map_err(|_| ParseError::CardAnswerIsIsEmpty)?;
    Ok((input, Card::new(question, answer)))
}

struct TextIsEmpty;

fn text_until_card_separator(input: &str) -> Result<(&str, &str), TextIsEmpty> {
    let (input, text) = split_text("\n/==", input);
    let text = text.trim();
    if text == "" {
        return Err(TextIsEmpty);
    }
    Ok((input, text))
}

fn text_until_card_divider(input: &str) -> Result<(&str, &str), TextIsEmpty> {
    let (input, text) = split_text("\n/-", input);
    let text = text.trim();
    if text == "" {
        return Err(TextIsEmpty);
    }
    Ok((input, text))
}

fn split_text<'a>(separator: &str, input: &'a str) -> (&'a str, &'a str) {
    let (input, text) = match input.find_substring(separator) {
        Some(index) => input.split_at(index),
        None => ("", input),
    };
    input.strip_prefix("\n/==").unwrap_or(input);
    (input, text)
}

fn line(input: &str) -> (&str, &str) {
    match input.find_substring("\n") {
        Some(index) => {
            let (input, line) = input.split_at(index);
            let input = &input[1..];
            (input, line)
        }
        None => ("", input),
    }
}
