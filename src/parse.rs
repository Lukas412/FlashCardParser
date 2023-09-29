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
    let (text, input) = match input.find_substring(separator) {
        Some(index) => input.split_at(index),
        None => (input, ""),
    };
    let input = input.strip_prefix(separator).unwrap_or(input);
    (input, text)
}

#[cfg(test)]
mod test {
    use crate::parse::{card, cards};
    use crate::{Card, ParseError};
    use std::vec;

    #[test]
    fn can_parse_multiple_cards() {
        let input = "First Card Question\n/-\nFirst Card Answer\n/==\nSecond Card Question\n/-\nSecond Card Answer";
        let expected = Ok(vec![
            Card::new("First Card Question", "First Card Answer"),
            Card::new("Second Card Question", "Second Card Answer"),
        ]);
        let actual = cards(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_card() {
        let input = "Card Question\n/-\nCard Answer";
        let expected = Ok(("", Card::new("Card Question", "Card Answer")));
        let actual = card(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_card_with_separator_at_end() {
        let input = "Card Question\n\
             /-\n\
             Card Answer\n\
             /==";
        let expected = Ok(("", Card::new("Card Question", "Card Answer")));
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
        let input = "/-\nAnswer\n/==";
        let expected = Err(ParseError::CardQuestionIsEmpty);
        let actual = card(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn cannot_parse_card_with_empty_answer() {
        let input = "Question/-\n\n/==";
        let expected = Err(ParseError::CardQuestionIsEmpty);
        let actual = card(input);
        assert_eq!(expected, actual);
    }
}
