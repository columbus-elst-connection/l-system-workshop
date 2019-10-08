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
    ( $(let $name:ident = $parser:expr),+ => $finish:expr ) => {{
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

### `macro_export` attribute
Let's start at the top. The `macro_export` attribute tells us that we would like
to make this macro available whenever the crate is in scope.

### `macro_rules!` macro
The way to define a macro is by using the `macro_rule` macro. It will accept a
name and shapes that the macro should accepts and how they are translated.

In our case are creating a macro called `sequence`.

### The pattern
The pattern our macro accepts is defined next. It is 

```
( $(let $name:ident = $parser:expr),+ => $finish:expr )
```
First let's focus on `let $name:ident = $parser:expr`. It expresses how we want
to string a results of parser. This pattern will try to match expressions like

```
let a = character('A')
```

first the literal `let` next something that looks like an identifier, `a` in this
case. if it succeeds bind the identifier to the `$name` variable. Next comes a
literal `=` followed by something that looks like any Rust expression,
`characcter('A')` in our example, and bind that expression to the `$parser`
variable.

Notice that that entire pattern is surrounded by `$( ),+`. The plus tells us
that we need at least one, but are allowed more repetition of what ever pattern
is between the brackets, separated by an (final optional) comma.

Next there is a literal `=>` followed by something that looks like any rust
expression. If that matches bind the expression to the `$finish` expression.

### The substitutions
Declarative macros in Rust work by taking the pattern and providing a
substitution, that needs to adhere to some rules. 

```
    {{
        |input| {
            let rem = input;
            $(
                let ($name, rem) = $parser.parse(rem)?;
            )*
            let result = $finish;
            Ok((result, rem))
        }
    }};
```

Now this substitution will be replaced with the data that is gathered when
matching the syntax. We notice that it will return a lambda expression that
accepts input, and returns a pair of result and the remaining input. I.e. it is
a parser.

What is returned is in effect handled by 
           
```
            $(
                let ($name, rem) = $parser.parse(rem)?;
            )*
```

The `$( )*` will output its contents for each of the matching parts.

## Exercises
1. Implement and test the `sequence!` macro.
2. Often white space is not that interesting. Create a macro that allows and
   ignores whitespace between the different parsers.
3. Sometimes we want to create some variables and move them in the closure.
   Create a `move_sequence` macro that moves the environment.
