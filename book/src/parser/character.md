# Parsing a `char`
In this chapter we will spend some time explaining how to create a `Parser`
capable of parsing a `char`. We will go through all the motions and explain
all the steps.

Our goal for this chapter is to make the following test pass.

```
    fn parse_a_character() {
        let input = "ABCD";
        let parser = character('A');

        let actual = parser.parse(input);

        let expected = Ok(('A', "BCD"));
        assert_eq!(actual, expected);
    }
```

So let's get started.

## Character
Thinking about the challenge of parsing a `char` one could notice the following
things. Giving a certain `&str`, we need to

1. Check if it starts with the correct character.
2. If it does, return it and the rest of the `&str`.
3. If it does not, report an error.

In order to be able to implement the `Parser` trait, there first needs to be
something we can implement the `Parser` trait for. This thing will need to keep
track of what character to parse.

We will use a struct `Character` for that. We will give it a field that tracks
the intended character to match.

```
pub struct Character {
  character_to_match: char,
}
```

## Impl Parser
Let's implement `Parser` for the `Character` struct. We will be guided by the outline above

```
impl<'a> Parser<'a, char> for Character {
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), ParseError> {
        if input.starts_with(self.character_to_match) {
            Ok((self.character_to_match, &input[1..]))
        } else {
            Err(ParseError::ExpectingCharacter(self.character_to_match))
        }
    }
}
```

The first two lines are the signature needed to adhere to the traits contract.
Next we check if the input starts with the character to match. If it does, we
return a succesful parse with the matched character and the remaining input.
Otherwise we announce an error, which we also need to add to our `ParseError`
enum.

```
#[derive(Debug, PartialEq)]
pub enum ParseError {
    GenericError,
    ExpectingCharacter(char),
}
```

## Constructor
It is convenient to create a _constructor_ for our `Character` struct. This eases the
burden of creating a `Character` parser.

```
impl Character {
    pub fn new<'a>(character_to_match: char) -> impl Parser<'a, char> {
        Self { character_to_match }
    }
}
```

Notice the return type of our constructor. It hides the implementation details.
It only tells you that you can expect to get something that implements a `Parser`
that returns a `char` on success.

This allows us greater flexibility in the future. We can easily change the structure
of Character without changing the constructor, or code that relies on it.

## Factory
An other convience is the use of a [factory][]. A factory is a pattern that hides the
details of how an struct is created. It serves yet an other purpose, to use a name that
reflects the intention, instead of exposing the mechanism of construction.

So instead of writing `Character::new('a')`, we want to write `character('a')`. Under
the covers this is done by using the constructor for `Character`.

```
pub fn character<'a>(character_to_match: char) -> impl Parser<'a, char> {
    Character::new(character_to_match)
}
```

## Summary
Since the `Character` parser is an example of how parser can be created we are going to
summarize the steps.

1. Choose a datatype that allows the parser to do it's job.
2. Implement `Parser` for it.
3. Create a _constructor_.
4. Create a factory.

The `Character` parse was build of a struct. Implementing `Parser` was done by translating
the parsing process into rust. And constructor and factory were created for ease of use.

## Exercises
1. Implement the `Character` parser.
2. Write some tests to check your implementation.

[factory]: https://en.wikipedia.org/wiki/Factory_method_pattern