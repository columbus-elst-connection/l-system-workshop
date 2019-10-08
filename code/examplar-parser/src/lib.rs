extern crate api;

mod framework;

use api::RenderConfig;
use self::framework::{Parser, ParseError, literal, character, newline, number};

pub fn render_config<'a>() -> impl Parser<'a, RenderConfig>  {
    sequence!{
        let _header = header("config"),
        let step = key_value("step", number()),
        let angle = key_value("angle", number())
        =>
        RenderConfig { step: step, angle: angle }
    }
}

pub fn header<'a, S>(identifier: S) -> impl Parser<'a, ()> where S: Into<String> {
    let header_name = identifier.into();
    move_sequence!{
        let _identifier = literal(&header_name), 
        let _colon = character(':'),
        let _newline = newline()
        =>
        ()
    }
}

pub fn key_value<'a, S, T, P>(identifier: S, parser: P) -> impl Parser<'a, T> where S: Into<String>, P: Parser<'a, T> + Sized {
    let key_name = identifier.into();
    move_sequence_ignore_spaces!{
        let _identifier = literal(&key_name),
        let _equal = character('='),
        let value = parser,
        let _newline = newline()
        =>
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_header_is_parsed() {
        let input = r##"a_header:
        "##;

        let (actual, _rem) = header("a_header").parse(input).expect("to parse a header");

        let expected = ();
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_key_value_is_parsed() {
        let input = r##"angle = 100
        "##;

        let (actual, _rem) = key_value("angle", number()).parse(input).expect("to parse a key value");

        let expected = 100;
        assert_eq!(actual, expected);
    }


    #[test]
    fn valid_render_config_is_parsed() {
        let input = r##"config:
        step = 8
        angle = 45
        "##;

        let (actual, _rem) = render_config().parse(input).expect("to parse a configuration");

        let expected = RenderConfig { step: 8, angle: 45 };
        assert_eq!(actual, expected);
    }
}