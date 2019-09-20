# Between
We are capable of parsing, but all the parsers we have seen
only parse a single character. Let's say we want to parse a
number instead of a single digit.

For that we our going to look at the `Between` parser. We
expect it to work like the following.

```
    #[test]
    fn parse_between_2_and_4_digits() {
        let input = "12345";
        let parser = between(2, 4, any(|c: char| c.is_ascii_digit()));

        let actual = parser.parse(input);

        let expected = Ok((vec!['1', '2', '3', '4'], "5"));
        assert_eq!(actual, expected);
    }
```

## Considerations
By now the strategy to creating a parser is not new. Now we have
to think about what we need to represent this parser. We need
some sort of lower bound, an upper bound and a parser that we
want to parse between the lower and upper bound times.

The lower bound can be just a number. Because you can not parse
less than zero times, negative numbers are not necessary. The type
for the lower bound can be any unsigned number. We will settle for
`u8`.

The upper bound is a little trickier. There is no natural limit,
we could be interested in all characters that match a certain parser.

To accomadate this we are going to introduce a custom enum `Limit`.

```
pub enum Limit {
  At(u8),
  Infinity,
}
```

This way we can specify either an unlimited bound, or a bound at a
specific value.

Because we likely will want to check if the upper bound is reached,
we introduce a method that allows use to check that.

```
impl Limit {
  pub fn is_bigger_then(&self, n: u8) -> bool {
    match self {
      Limit::At(threshold) => threshold > &n,

      Limit::Infinity => true,
    }
  }
}
```

That are almost all the considerations for the `Between` struct we
need. Let's see if you can come up with an implementation. Do note
that you probably need to think about `PhantomData` at some point.

## Impl Parser
So we have a struct that tracks the lower limit, the upper limit and
the base parser. The implementation strategy is.

1. Try to parse as many as the lower limit of items, accumulating the
   result.
2. If that fails, report the error.
3. If it succeeds, go on to the optional phase.
4. Try to parse up to the upper limit of items.
5. If that fails, cut your losses and return the accumulated result.
6. If that succeeds, return the accumulated result. 

## Constructor & Factory
Create an appropriate constructor and factories. See the exercises
for some factory suggestions.

## Summary
This combinator is very versatile. It can accomodate a lot of different
scenarios, depending on how you construct it. Writing factories that
reflect your needs is always a good strategy.

On should write code that one wants and write implementations to make
that happen.

## Exercises
1. Implement the `Between` parser.
2. Write some tests to check your implementation.
3. Introduce some other factories in terms of `Between`

* `atleast`, will parse at least a certain number of items, but
   tries to parse as many as it can from the input.
* `many`, will parse as many items as it can from the input.

4. Think and discuss how to do better error reporting. Implement
   your strategy.
