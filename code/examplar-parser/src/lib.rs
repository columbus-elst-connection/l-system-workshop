pub fn parse(_input: &str) -> Result<(), ParseError> {
  Ok(())
}

pub enum ParseError {
    GenericError
}

pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError>;
}

impl <'a, T, F> Parser<'a, T> for F where F: Fn(&'a str) -> Result<(T, &'a str), ParseError> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError> {
        self(input)
    }
}