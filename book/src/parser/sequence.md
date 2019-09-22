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
