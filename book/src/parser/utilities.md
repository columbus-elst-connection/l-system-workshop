# Utilities
This chapter introduces some convenient parsers that we will use when we will be
parsing more complex structures.

## skip
You are not always interested in what you are parsing. Whitespace is a good
example. The `skip` factory creates a parser that allows you to transform a
parser into one that ignores the result.

```
    #[test]
    fn parse_skip_many_spaces() {
        let input = "           next";
        let parser = skip(many(character(' ')));

        let actual = parser.parse(input);

        let expected = Ok(((), "next"));
        assert_eq!(actual, expected);
    }
```

An implementation can be given in terms of the primitives and the combinators
already given.

## space & spaces
A useful building block, to ignore, are white space. The following would be a
nice addition to our utility belt. 

```
    #[test]
    fn parse_spaces() {
        let input = "           \tnext";
        let parser = spaces();

        let actual = parser.parse(input);

        let expected = Ok(((), "next"));
        assert_eq!(actual, expected);
    }
```

Don't forget about the fact that a *tab* character also counts as white space.

## newline
It is nice to be able to know when a line ends. Line ends are signified by
newline. Depending on the that could be one of `\n`, `\r` or `\r\n`.

## digit
This parser is an intermediate parser for numbers. It would be nice to be able
to parse a digit.

## number
Many digits strung together form a number.

## Exercises
1. Implement `skip` parser. Hint: take a look at `map`
2. Implement `space` and `spaces` parser. Hint: don't forget about `skip`.
3. Implement `newline`. Hint: take a look at `one_of`.
4. Implement `digit`. Hint: take a look at `any`.
5. Implement `number`. Hint: take a look at `many` and don't forget about `digit`.
