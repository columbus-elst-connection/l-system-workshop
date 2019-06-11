pub fn parse(_input: &str) -> Result<(), ParseError> {
  Err(ParseError::GenericError)
}

pub enum ParseError {
    GenericError
}
