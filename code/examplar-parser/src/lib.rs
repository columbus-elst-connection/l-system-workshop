extern crate api;

mod framework;

use api::{Rule, RenderConfig, LSystemRules, LSystem};
use self::framework::{Parser, ParseError, literal, character, newline, number, at_least, many, any, blank_lines, end};

pub fn parse(input: &str) -> Result<LSystem<char>, ParseError> {
    let parser = system();
    parser
        .parse(input)
        .map(|(l_system, _rest)| l_system)
}


pub fn system<'a>() -> impl Parser<'a, LSystem<char>>  {
    end(sequence!{
        let config = render_config(),
        let _ignore1 = blank_lines(),
        let rules = rules(),
        let _ignore2 = blank_lines()
        =>
       LSystem { render_config: config, axiom: rules.0, rules: rules.1 }
    })
}

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

pub fn rules<'a>() -> impl Parser<'a, (Vec<char>, LSystemRules<char>)>  {
    sequence!{
        let _header = header("rules"),
        let axiom = key_value("axiom", at_least(1, symbol())),
        let rules = many(rule())
        =>
       (axiom, LSystemRules::from_rules(rules))
    }
}

pub fn symbol<'a>() -> impl Parser<'a, char> {
    any(is_symbol)
}

fn is_symbol(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '[' || character == ']' || character == '+' || character == '-'
}

pub fn rule<'a> () -> impl Parser<'a, Rule<char>> {
    sequence_ignore_spaces!{
        let key = symbol(),
        let _becomes = literal("=>"),
        let production = at_least(1, symbol()),
        let _newline = newline()
        =>
        Rule::new(key, production)
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

    #[test]
    fn valid_rule_is_parsed() {
        let input = "F => FF\n";
        let (actual, _rem) = rule().parse(input).expect("to parse rule");

        let expected = Rule::new('F', vec!['F', 'F']);
        assert_eq!(actual, expected);
    }


    #[test]
    fn valid_rules_is_parsed() {
        let input = r##"rules:
        axiom = F
        F => FF
        "##;

        let (actual, _rem) = rules().parse(input).expect("to parse rules");

        let expected = (vec!['F'], LSystemRules::from_rules(vec![Rule::new('F', vec!['F', 'F'])]));
        assert_eq!(actual, expected);
    }

    #[test]
    fn valid_system_is_parsed() {
        let input = r##"config:
        step = 8
        angle = 45
        rules:
        axiom = A
        A => BA
        B => A
        "##;

        let (actual, _rem) = system().parse(input).expect("to parse a system");

        let expected = LSystem {
            render_config: RenderConfig { step: 8, angle: 45 },
            axiom: vec!['A'],
            rules: LSystemRules::from_rules(vec![Rule::new('A', vec!['B', 'A']), Rule::new('B', vec!['A'])])
        };
        assert_eq!(actual, expected);
    }
}
