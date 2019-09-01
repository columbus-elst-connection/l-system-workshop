use std::marker::PhantomData;

pub fn parse(_input: &str) -> Result<(), ParseError> {
  Ok(())
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    GenericError,
    ExpectingCharacter(char),
    ExpectingPredicate,
    EndOfInput,
}

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError>;
}

impl <'a, T, F> Parser<'a, T> for F where F: Fn(&'a str) -> Result<(T, &'a str), ParseError> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError> {
        self(input)
    }
}

pub struct Character {
  character_to_match: char,
}

impl<'a> Parser<'a, char> for Character {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), ParseError> {
        if input.starts_with(self.character_to_match) {
            Ok((self.character_to_match, &input[1..]))
        } else {
            Err(ParseError::ExpectingCharacter(self.character_to_match))
        }
    }
}

impl Character {
    pub fn new<'a>(character_to_match: char) -> impl Parser<'a, char> {
        Self { character_to_match }
    }
}

pub fn character<'a>(character_to_match: char) -> impl Parser<'a, char> {
  Character::new(character_to_match)
}

pub struct Any<F> where F: Fn(char) -> bool + Sized {
    predicate: F,
}

impl<'a, F> Parser<'a, char> for Any<F> where F: Fn(char) -> bool + Sized {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), ParseError> {
        let character = input.chars().next();
        match character {
            Some(c) => {
                if (self.predicate)(c) {
                    Ok((c, &input[1..]))
                } else {
                    Err(ParseError::ExpectingPredicate)
                }
            },

            None => {
                Err(ParseError::EndOfInput)
            }
        }
    }
}

impl<F> Any<F> where F: Fn(char) -> bool + Sized {
    pub fn new(predicate: F) -> Self {
        Any { predicate }
    }
}

pub fn any<'a, F>(predicate: F) -> impl Parser<'a, char> where F: Fn(char) -> bool + Sized {
    Any::new(predicate)
}

pub struct Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    parser: P,
    map: F,
    phantom: PhantomData<&'a I>,
}

impl<'a, I, O, P, F> Parser<'a, O> for Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    fn parse(&self, input: &'a str) -> Result<(O, &'a str), ParseError> {
        let attempt = self.parser.parse(input);
        attempt.map(|(v, rest)|{ ((self.map)(v), rest)})
    }
}

impl<'a, I, O, P, F> Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    pub fn new(parser: P, map: F) -> Self {
        Map { parser, map, phantom: PhantomData }
    }
}

pub fn map<'a, I, O, P, F>(parser: P, map: F) -> impl Parser<'a, O> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    Map::new(parser, map)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_character() {
        let input = "ABCD";
        let parser = character('A');

        let actual = parser.parse(input);

        let expected = Ok(('A', "BCD"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_any_digit() {
        let input = "0123";
        let parser = any(|c: char| c.is_ascii_digit());

        let actual = parser.parse(input);

        let expected = Ok(('0', "123"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_any_digit_as_number() {
        let input = "1230";
        let parser = map(
          any(|c: char| c.is_ascii_digit()), 
          |c: char| c.to_digit(10).unwrap_or(0));

        let actual = parser.parse(input);

        let expected = Ok((1, "230"));
        assert_eq!(actual, expected);
    }
}