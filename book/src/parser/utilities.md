# Utilities
This chapter introduces some convenient parsers that we will use when we will be
parsing more complex structures.

## Skip
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

## Exercises
1. Implement `skip` parser. Hint: take a look at `map`
