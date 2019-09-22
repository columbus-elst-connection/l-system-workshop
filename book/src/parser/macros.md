# Macros
We have a fair amount of basic parsers and lovely combinators for them. But
there is one thing missing. All the parsers we have so far can only parse one
thing.

Give a parser an input, it will try do its job, return the result and the rest
of the input. But if we want to parse complex structures, we need to be able to
proceed parsing an that rest of the input. We need to be able to chain different
parser together, one after the other.

## What we want to achieve
Let's say we want to parse something that looks like a L-system rule. Remember,
it starts with a symbol, followed by a literal `=>` and at last a sequence of
symbols.

What needs to be done is that we feed the remainder of one parser into the next.
Something along the lines of

```
let rem = input;
let (symbol, rem) = symbol_parser.parse(rem)?
let (_, rem) = literal('=>').parse(rem)?
let (production, rem) = at_least(1, symbol_parser).parse(rem)?
```

Once we have the symbol and its production we can combine them to form a rule.

Although this is not hard, there is a lot of plumbing that is neither
interesting or exciting. We would rather focus on chaining the parsers and
combing their results.

## Declarative Macros
Rust has a great mechanism to aid in our search for expressiveness. It are
[declarative macros](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming).

Declarative macros

> allow you to write something similar to a Rust match expression. As discussed
> in Chapter 6, match expressions are control structures that take an
> expression, compare the resulting value of the expression to patterns, and
> then run the code associated with the matching pattern. Macros also compare a
> value to patterns that are associated with particular code: in this situation,
> the value is the literal Rust source code passed to the macro; the patterns
> are compared with the structure of that source code; and the code associated
> with each pattern, when matched, replaces the code passed to the macro. This
> all happens during compilation 

They are ideal for our situation.
