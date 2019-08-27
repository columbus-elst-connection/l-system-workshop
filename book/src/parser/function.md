# Functions are Parsers!
With our `Parser` trait in place we can start implementing it. Thinking about
the trait one implementation springs to mind.

If we have a function, or a lambda expression for that matter, that has the
right signature. I.e. accepts a `&str` and returns a 
`Result<(T, &str), ParseError>`, in some sense it is a parser. We just have to
apply the function! 

Let's start with that implementation.

```
impl <'a, T, F> Parser<'a, T> for F where F: Fn(&'a str) -> Result<(T, &'a str), ParseError> {
    fn parse(&self, input: &'a str) -> Result<(T, &'a str), ParseError> {
        self(input)
    }
}
```

## Exercises 
1. Write the implementation of the `Parser` trait for `Fn(&'a str) -> Result<(T, &'a str), ParserError>`.