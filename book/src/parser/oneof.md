# OneOf
Sometimes you want to have a choice. E.g. an expression could either be a number
or a variable. So when parsing an expression you want the option to parse one of
a number or variable.

This is where the `OneOf` parser comes in. It will pass the following test.

```
    #[test]
    fn parse_one_of_a_or_b() {
        let input = "a1";
        let parser = one_of(vec![character('a'), character('b')]);

        let actual = parser.parse(input);

        let expected = Ok(('a', "1"));
        assert_eq!(actual, expected);
    }
```

## Considerations
Since the `Parser` trait can return only a single thing, all of our options in
the `OneOf` parser should adhere to the same contract.


## Impl Parser
Once you have your options our parse strategy would be to try each of our
options in turn until one of them succeed. If it succeed return the parsed
result and the remaining input. If none of the options succeed, report an error.

## Constructor & Factories
Since there is not very much going on with constructors or factories. From now
on we will not mention them any more.

## Exercises
1. Implement the `OneOf` parser.
2. Write some test to check your implementation.
3. Think and discuss to do better error handling.
