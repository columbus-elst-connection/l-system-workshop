mod framework;

use self::framework::ParseError;

pub fn parse(_input: &str) -> Result<(), ParseError> {
  Err(ParseError::GenericError)
}
