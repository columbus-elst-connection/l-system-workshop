# Parsing any `char`
In a previous chapter we created a parser that could parse a single
specific character. An other useful parser would parse a string
would start with any of a group of characters.

To be more precise. Let's say that we would like to parse a `&str` if
it starts with a digit. What we would like to achieve is to pass the 
following test

```
    #[test]
    fn parse_any_digit() {
        let input = "0123";
        let parser = any(|c: char| c.is_ascii_digit());

        let actual = parser.parse(input);

        let expected = Ok(('0', "123"));
        assert_eq!(actual, expected);
    }
```

We will go through all the motions of creating a parser once more.

## Any
We will create a struct that will contain everything it needs to
make our wish come through. For this we need to be able to store
a _predicate_, a function that will decide on the characters we
expect.

```
pub struct Any<F> where F: Fn(char) -> bool + Sized {
    predicate: F,
}
```

Here `F` is an alias for the `Fn(char) -> bool + Sized` type.
Our predicate, is a function that accepts a character and tells
use if it does or does not fall in our category. The `Sized` restriction
is necessary because we want to store it in our field.

## Impl Parser
Next we our going to implement the `Parser` trait. The basic idea is,
given our input

1. Check if it starts with a character from our category.
2. If it does, return it and the rest of the input.
3. If it does not, return an error.

The difference with the `Character` parser is that `String` does not have
a convenience method like `starts_with` that we can use. Instead we can use
an _iterator_ over the characters of our input. See below for details.

```
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
```

We ask our input for an iterator over its characters. Match on the first character
if it exists, and if it does, check to see if it is in the category. Notice that 
we introduced two new `ParseError`s.

```
#[derive(Debug, PartialEq)]
pub enum ParseError {
    GenericError,
    ExpectingCharacter(char),
    ExpectingPredicate,
    EndOfInput,
}
```

# Constructor and Factory
The final two steps are to create a constructor and a convenient factory.

```
impl<F> Any<F> where F: Fn(char) -> bool + Sized {
    pub fn new(predicate: F) -> Self {
        Any { predicate }
    }
}

pub fn any<'a, F>(predicate: F) -> impl Parser<'a, char> where F: Fn(char) -> bool + Sized {
    Any::new(predicate)
}
```

## Summary
Just like the first parser we made, we followed a predictable course.

1. Choose a datatype that allows the parser to do it's job.
2. Implement `Parser` for it.
3. Create a _constructor_.
4. Create a factory.

This pattern will be repeated many times.

## Exercises
1. Implement the `Any` parser.
2. Write some tests to check your implementation.
