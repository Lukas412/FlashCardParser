use crate::{Card, Topic};
use derive_more::{Display, Error, From};
use nom::FindSubstring;

#[derive(Debug, Clone, Display, Error, From, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParseError<'a> {
    TopicTitleIsEmpty,
    TopicTitleIsMultipleLinesLong { title: &'a str },
    CardQuestionIsEmpty,
    CardAnswerIsIsEmpty,
}

pub(crate) fn topic<'a>(input: &'a str) -> Result<Topic<'a>, ParseError> {
    let input = input.trim();
    let (input, title) = topic_title(input)?;
    let cards = cards(input)?;
    Ok(Topic::new(title, cards))
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
    let (text, input) = match input.find_substring(separator) {
        Some(index) => input.split_at(index),
        None => (input, ""),
    };
    let input = input.strip_prefix(separator).unwrap_or(input);
    (input, text)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Card, ParseError};
    use std::vec;

    #[test]
    fn can_parse_topic() {
        let input = "Title\n/==\nQuestion\n/-\nAnswer\n/==";
        let expected = Ok(Topic::new("Title", vec![Card::new("Question", "Answer")]));
        let actual = topic(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_topic_title() {
        let input = "Title\n/==";
        let expected = Ok(("", "Title"));
        let actual = topic_title(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_topic_title_with_empty_lines() {
        let input = "Title\n\n\n/==";
        let expected = Ok(("", "Title"));
        let actual = topic_title(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_topic_title_when_empty() {
        let input = "\n\n/==";
        let expected = Err(ParseError::TopicTitleIsEmpty);
        let actual = topic_title(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_topic_title_when_multiple_lines_long() {
        let input = "Title\nWith Second Line\n\n/==";
        let expected = Err(ParseError::TopicTitleIsMultipleLinesLong {
            title: "Title\nWith Second Line",
        });
        let actual = topic_title(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_multiple_cards() {
        let input = "First Question\n/-\nFirst Answer\n/==\nSecond Question\n/-\nSecond Answer";
        let expected = Ok(vec![
            Card::new("First Question", "First Answer"),
            Card::new("Second Question", "Second Answer"),
        ]);
        let actual = cards(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_card() {
        let input = "Question\n/-\nAnswer";
        let expected = Ok(("", Card::new("Question", "Answer")));
        let actual = card(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_card_with_separator_at_end() {
        let input = "Question\n/-\nAnswer\n/==";
        let expected = Ok(("", Card::new("Question", "Answer")));
        let actual = card(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_card_with_texts_inline_with_divider() {
        let input = "Question\n/-Answer\n";
        let expected = Ok(("", Card::new("Question", "Answer")));
        let actual = card(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_card_with_empty_question() {
        let input = "\n/-\nAnswer\n/==";
        let expected = Err(ParseError::CardQuestionIsEmpty);
        let actual = card(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_card_with_empty_answer() {
        let input = "Question/-\n\n/==";
        let expected = Err(ParseError::CardAnswerIsIsEmpty);
        let actual = card(input);
        assert_eq!(expected, actual);
    }
}
