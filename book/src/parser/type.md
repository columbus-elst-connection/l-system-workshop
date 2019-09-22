# Looking at the type
Before we start out it is good to think what we want to achieve. This allows us
to reason about the constraints and capture those in our types.

A parser reads input and transforms that into some data structure. This already
provides us with a decision. I.e. what type is our input?

We want to make things easy for our selves and since string literals have type
`&str` we our going to pick that as our input type.

We don't really know what output type is going to be, put we do know that the
parser could fail. Take a look at the following code

```lsystem
F -> FF
```

Although it looks like an L-system, it not quite an L-system. At least not in
the form we expect in the L-system section. The arrow is wrong!

Because the data-structure could be anything, we better make it a generic
parameter. So our first guess for the output type could be
`Result<T, ParserError>`, where `ParserError` is defined as

```rust
#[derive(Debug, PartialEq)]
pub enum ParseError {
    GenericError,
}
```

## Combinator
Although our first guess isn't that far off, it is lacking some flexibility.
What we like to achieve is to easily combine different kind of parsers into a
more complex one.

With this goal in mind, we would like to pass information about what part of the
input still needs parsing. Because we want to feed that into a different parser,
that type should be `&str`, our input type.

So our second guess is `Result<(T, &str), ParseError>`.

A Parser returns either a tuple of the parsed result and the rest of the
input, or it returns an error. 

## Trait
We now can create a `trait` that describes the contract our parsers have. It is
little more than our consideration, with the correct use of lifetimes to keep
the compiler happy.

```rust
pub trait Parser<'a, T> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError>;
}
```

So a Parser is anything that has a `parse` method of the right signature.

## Exercises
1. Implement the `Parser` trait and the `ParseError` enum in `src/framework.rs`.
