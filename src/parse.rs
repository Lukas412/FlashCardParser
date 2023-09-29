use crate::Topic;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum ParseError<'a> {
    TopicTitleIsEmpty,
    CardQuestionIsEmpty,
    CardAnswerIsIsEmpty,
    TopicTitleIsMutltipleLinesLong { title: &'a str },
}

pub(crate) fn topic(input: &str) -> Result<(&str, Topic), ParseError> {
    let (input, title) = topic_title(input)?;
    let (input, cards) = cards(input)?;
    Ok((input, Topic { title, cards }))
}

fn topic_title(input: &str) -> Result<(&str, &str), ParseError> {
    let (input, text) =
        text_until_card_separator(input).map_err(|_| ParseError::TopicTitleIsEmpty)?;
    if text.contains("\n") {
        return Err(ParseError::TopicTitleIsMutltipleLinesLong { title: text });
    }
    Ok((input, text))
}

fn cards(input: &str) -> Result<(&str, &str), ExpectedCard> {
    let input = take_card_separator(input)?;
    let results = Vec::new();
}

fn card(input: &str) -> Result<(&str, &str), ExpectedCard> {
    let input = take_card_separator(input)?;
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

fn split_text(separator: &str, input: &str) -> (&str, &str) {
    let (input, text) = match input.find_substring("\n==") {
        Some(index) => input.split_at(index),
        None => ("", input),
    };
    input.strip_prefix("\n/==").unwrap_or(input);
    (input, text)
}

fn take_card_divider(input: &str) -> Result<&str, ExpectedCardDivider> {
    let (input, line) = trimed_line(input);
    if line != "&" {
        return Err(ExpectedCardDivider { line });
    }
    Ok(input)
}

fn take_empty_lines(mut input: &str) -> &str {
    loop {
        let (remaining, line) = trimed_line(input);
        if line == "" || input == "" {
            return input;
        }
        input = remaining;
    }
}

fn trimed_line(input: &str) -> (&str, &str) {
    let (input, line) = line(input);
    let line = line.trim();
    (input, line)
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
