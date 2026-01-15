use crate::errors::PatternError;
#[derive(Debug, PartialEq, Eq)]
enum Card {
    Digit,         // \d
    Literal(char), // abcdeAbcdzzz231237 etc
    Alphanumeric, // \w
}

pub struct Pattern {
    expression: Vec<Card>,
}

impl Pattern {
    pub fn new(input: &str) -> Result<Self, PatternError> {
        let mut expression = vec![];
        let mut input_chars = input.chars().peekable();

        while let Some(curr_char) = input_chars.next() {
            match curr_char {
                '\\' => {
                    if let Some(next_char) = input_chars.next() {
                        let card = Self::parse_class(&next_char);
                        expression.push(card);
                    } else {
                        return Err(PatternError::NoClassFound);
                    }
                }
                '^' => unimplemented!("`^`"),
                '$' => unimplemented!("`$`"),
                '[' => unimplemented!("`[`"),
                // literal
                c => expression.push(Card::Literal(c)),
            }
        }

        Ok(Self { expression })
    }

    fn parse_class(c: &char) -> Card {
        match c {
            'd' => Card::Digit,
            '\\' => Card::Literal('\\'),
            'w' => Card::Alphanumeric,
            another => panic!("not supported yet: {another}"),
        }
    }

    pub fn is_match(&self, input: &str) -> bool {
        if input.is_empty() || self.expression.is_empty() {
            return false;
        }

        for input_char in input.chars() {
            for card in self.expression.as_slice() {
                match card {
                    Card::Digit => {
                        if input_char.is_ascii_digit() {
                            return true;
                        } else {
                            continue;
                        }
                    }
                    Card::Literal(literal) => {
                        if literal == &input_char {
                            return true;
                        } else {
                            continue;
                        }
                    }
                    Card::Alphanumeric => {
                        if input_char.is_alphanumeric() || input_char == '_' {
                            return true;
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
        return false;
    }
}