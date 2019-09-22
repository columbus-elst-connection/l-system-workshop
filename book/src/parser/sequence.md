# Parsing a sequence
We would like to parse a sequence of things. First this, followed by that and
finally this thing again. At the moment we have no way of describing that. Let's
remedy that.

What we want to achieve is to make the following test pass.

```
    #[test]
    fn parse_a_sequence_of_parsers() {
        let parser = sequence!{
            let a = character('A'),
            let b = character('b')
            =>
            (a, b)
        };

        let (result, rem) = parser.parse("Ab").expect("failed to parse");
        
        assert_eq!(('A', 'b'), result);
        assert!(rem.is_empty());
    }
```

## What do we want
As stated in the Macros chapter, we want to sequence parsers without having to
write all the boilerplate. As detailed in the test code above, we want to focus
on the result of parsers and combine those results in interesting ways.

The pattern that we picked is, one or more lines of

```
let identifier = parser,
```

We would like to translate the above lines into something like

```
let (identifier, remainder) = parser.parse(remainder)?
```

I.e. pass the remaining input to the parser to parse, collect the result and
bind it to the identifier and rebind the remainder.

Once all the parser sequences have done their job, we would like to collect all
the parse results in some meaningful manner. E.g. if we have parsed an `'A'`
followed by a `'b'`, we would return a tuple containing both results. More
general we want to return some form of expression involving the parse results.

## Macros to the rescue
Luckily macros are well suited for the task. Look at the following code. We will
explain it in detail.

```
#[macro_export]
macro_rules! sequence {
    ($(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
        |input| {
            let rem = input;
            $(
                let ($name, rem) = $parser.parse(rem)?;
            )*
            let result = $finish;
            Ok((result, rem))
        }
    }};
}
```
