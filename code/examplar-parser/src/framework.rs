use std::marker::PhantomData;

pub fn parse(_input: &str) -> Result<(), ParseError> {
  Ok(())
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    GenericError,
    ExpectingCharacter(char),
    ExpectingPredicate,
    ExpectingOneOfToParse,
    ExpectingLiteral(String),
    EndOfInput,
    ExpectingToBeAtEndOfInput
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
        Self { predicate }
    }
}

pub fn any<'a, F>(predicate: F) -> impl Parser<'a, char> where F: Fn(char) -> bool + Sized {
    Any::new(predicate)
}

pub struct Literal<'p>(&'p str);

impl <'a, 'p> Parser<'a, &'a str> for Literal<'p> {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), ParseError> {
        if input.starts_with(self.0) {
            let len = self.0.len();
            let substr = &input[..len];
            let rem = &input[len..];
            Ok((substr, rem))
        } else {
            Err(ParseError::ExpectingLiteral(self.0.to_owned()))
        }
    }
}

pub fn literal(match_exactly: &str) -> Literal {
    Literal(match_exactly)
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
        Self { parser, map, phantom: PhantomData }
    }
}

pub fn map<'a, I, O, P, F>(parser: P, map: F) -> impl Parser<'a, O> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    Map::new(parser, map)
}

pub struct Between<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    lower_limit: u8,
    upper_limit: Limit,
    parser: P,
    phantom: PhantomData<&'a T>,
}

pub enum Limit {
  At(u8),
  Infinity,
}

impl Limit {
  pub fn is_bigger_then(&self, n: u8) -> bool {
    match self {
      Limit::At(threshold) => threshold > &n,

      Limit::Infinity => true,
    }
  }
}

impl<'a, T, P> Parser<'a, Vec<T>> for Between<'a, T, P> where P: Parser<'a, T> + Sized {
    fn parse(&self, input: &'a str) -> Result<(Vec<T>, &'a str), ParseError> {
        let mut result = vec![];
        let mut source = input;
        let mut count = 0;
        while count < self.lower_limit {
            let attempt = self.parser.parse(source);
            match attempt {
                Ok((value, rest)) => {
                    result.push(value);
                    source = rest;
                }

                Err(e) => {
                    return Err(e);
                }
            }
            count += 1;
        }
        while self.upper_limit.is_bigger_then(count) {
            let attempt = self.parser.parse(source);
            match attempt {
                Ok((value, rest)) => {
                    result.push(value);
                    source = rest;
                }

                Err(_) => {
                    break;
                }
            }
            count += 1;
        }
        Ok((result, source))
    }
}

impl<'a, T, P> Between<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    pub fn new(lower_limit: u8, upper_limit: Limit, parser: P) -> Self {
        Self { lower_limit, upper_limit, parser, phantom: PhantomData }
    }
}

pub fn between<'a, T>(lower_limit: u8, upper_limit: u8, parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> where T: 'a {
    Between::new(lower_limit, Limit::At(upper_limit), parser)
}

pub fn at_least<'a, T>(lower_limit: u8, parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> where T: 'a {
    Between::new(lower_limit, Limit::Infinity, parser)
}

pub fn many<'a, T>(parser: impl Parser<'a, T>) -> impl Parser<'a, Vec<T>> where T: 'a {
    at_least(0, parser)
}

pub struct OneOf<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    options: Vec<P>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, P> Parser<'a, T> for OneOf<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError> {
        for ref parser in &self.options {
            let attempt = parser.parse(input);
            if attempt.is_ok() {
                return attempt
            }
        }
        Err(ParseError::ExpectingOneOfToParse)
    }
}

impl<'a, T, P> OneOf<'a, T, P> where T: 'a, P: Parser<'a, T> + Sized {
    pub fn new(options: Vec<P>) -> Self {
        Self { options, phantom: PhantomData }
    }
}

pub fn one_of<'a, T, P>(options: Vec<P>) -> impl Parser<'a, T> where T: 'a, P: Parser<'a, T> + Sized {
    OneOf::new(options)
}

pub fn skip<'a, T, P>(parser: P) -> impl Parser<'a, ()> where T: 'a, P: Parser<'a, T> + Sized {
    map(parser, |_|{()})
}

pub fn space<'a>() -> impl Parser<'a, ()> {
    skip(one_of(vec![character(' '), character('\t')]))
}

pub fn spaces<'a>() -> impl Parser<'a, ()> {
    skip(many(space()))
}

pub fn newline<'a>() -> impl Parser<'a, ()> {
    skip(one_of(vec![literal("\n"), literal("\r\n"), literal("\r")]))
}

pub fn number<'a>() -> impl Parser<'a, u16> {
    map(at_least(1, digit()), to_number)
}

fn to_number(digits: Vec<char>) -> u16 {
    let number: String = digits.into_iter().collect();
    u16::from_str_radix(&number, 10).unwrap_or(0)
}

pub fn digit<'a>() -> impl Parser<'a, char> {
    any(|c| c.is_ascii_digit())
}

pub fn end<'a, T, P>(parser: P) -> impl Parser<'a, T> where T: 'a, P: Parser<'a, T> + Sized {
    move |input| {
        parser.parse(input).and_then(|(result, rem)|{
            if !rem.is_empty() {
                return Err(ParseError::ExpectingToBeAtEndOfInput)
            }
            Ok((result, rem))
        })
    }
}


#[macro_export]
macro_rules! sequence {
    ($(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
        |input| {
            let rem = input;
            $(
                let ($name, rem) = $parser.parse(rem)?;
            )*
            let result = $finish;
            Ok((result, rem))
        }
    }};
}

#[macro_export]
macro_rules! move_sequence {
    ($(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
        move |input| {
            let rem = input;
            $(
                let ($name, rem) = $parser.parse(rem)?;
            )*
            let result = $finish;
            Ok((result, rem))
        }
    }};
}


#[macro_export]
macro_rules! sequence_ignore_spaces {
    ($(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
        |input| {
            let rem = input;
            $(
                let (_, rem) = $crate::framework::spaces().parse(rem)?;
                let ($name, rem) = $parser.parse(rem)?;
            )*
            let (_, rem) = $crate::framework::spaces().parse(rem)?;
            let result = $finish;
            Ok((result, rem))
        }
    }};
}

#[macro_export]
macro_rules! move_sequence_ignore_spaces {
    ($(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
        move |input| {
            let rem = input;
            $(
                let (_, rem) = $crate::framework::spaces().parse(rem)?;
                let ($name, rem) = $parser.parse(rem)?;
            )*
            let (_, rem) = $crate::framework::spaces().parse(rem)?;
            let result = $finish;
            Ok((result, rem))
        }
    }};
}

pub fn blank_lines<'a>() -> impl Parser<'a, ()> {
    skip(many(blank_line()))
}

pub fn blank_line<'a>() -> impl Parser<'a, ()> {
    sequence!{
        let _spaces = spaces(),
        let _newline = newline()
        =>
        ()
    }
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

    #[test]
    fn parse_literal_arrow() {
        let input = "=> FF";
        let parser = literal("=>");

        let actual = parser.parse(input);

        let expected = Ok(("=>", " FF"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_between_2_and_4_digits() {
        let input = "12345";
        let parser = between(2, 4, any(|c: char| c.is_ascii_digit()));

        let actual = parser.parse(input);

        let expected = Ok((vec!['1', '2', '3', '4'], "5"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_one_of_a_or_b() {
        let input = "a1";
        let parser = one_of(vec![character('a'), character('b')]);

        let actual = parser.parse(input);

        let expected = Ok(('a', "1"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_skip_many_spaces() {
        let input = "           next";
        let parser = skip(many(character(' ')));

        let actual = parser.parse(input);

        let expected = Ok(((), "next"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_spaces() {
        let input = "          \tnext";
        let parser = spaces();

        let actual = parser.parse(input);

        let expected = Ok(((), "next"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_newline() {
        let input = "\n";
        let parser = newline();

        let actual = parser.parse(input);

        let expected = Ok(((), ""));
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_number() {
        let input = "1234";
        let parser = number();

        let actual = parser.parse(input);

        let expected = Ok((1234, ""));
        assert_eq!(actual, expected);
    }

    #[test]
    fn end_parser_should_not_allow_any_input() {
        let input = "1234 ";
        let parser = end(number());

        let actual = parser.parse(input);

        let expected = Err(ParseError::ExpectingToBeAtEndOfInput);
        assert_eq!(actual, expected);
    }


    #[test]
    fn parse_a_sequence_of_parsers() {
        let parser = sequence!{
            let a = character('A'),
            let b = character('b')
            =>
            (a, b)
        };

        let (result, rem) = parser.parse("Ab").expect("failed to parse");
        
        assert_eq!(('A', 'b'), result);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_a_sequence_of_parsers_ignoring_spaces() {
        let parser = sequence_ignore_spaces!{
            let a = character('A'),
            let b = character('b')
            =>
            (a, b)
        };
        
        let (result, rem) = parser.parse("   A\tb").expect("failed to parse");
        
        assert_eq!(('A', 'b'), result);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_blank_lines() {
        let input = "\n \n\t\n \t \n";
        let parser = blank_lines();

        let actual = parser.parse(input);

        let expected = Ok(((), ""));
        assert_eq!(actual, expected);
    }
}