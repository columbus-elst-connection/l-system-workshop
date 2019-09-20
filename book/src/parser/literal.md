# Parsing a Literal String
Often you know exactly what you are looking for. E.g. when separating the the
symbol for its production we use `=>`. I would come in handy if there was a
parser that could do that job.

```
    #[test]
    fn parse_literal_arrow() {
        let input = "=> FF";
        let parser = literal("=>");

        let actual = parser.parse(input);

        let expected = Ok(("=>", " FF"));
        assert_eq!(actual, expected);
    }
```

## Literal
The only thing that we need to keep track of is the string that we are
interested in. Rust provides various ways to model your data. [Tuple
structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)
are a convenient way for our use case. It allows one to name a structure by
describing the fields in a tuple.

```
pub struct Literal<'p>(&'p str);
```

Note that we have a tuple with only one field. It does not have a name so we
need to reference it by index.

## Impl Parser
Parsing with a `Literal` amounts to the following steps

1. Check if the input starts with our literal.
2. If it does, chop of the parsed string and return it together with the
   remaining input
3. If it does not, report an error.

This translate into code as follows.

```
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
```

Take a look at the expected result. It has type `&'a str`. We will return a
slice of the input, so the lifetime should match up.

## Constructor
Since tuple structs come with a build int constructor, we do not need to do
anything here.

## Factory
The factory for the `Literal` parser can just be a function `literal` to returns
the parser

```
pub fn literal(match_exactly: &str) -> Literal {
    Literal(match_exactly)
}
```

## Exercises
1. Implement the `Literal` parser.
2. Create some tests to verify your implementation.
