# Map
The first combinator we are going describe is the `Map` combinator.
Let's say we have a parser to parse a digit, i.e.
`any(|c: char| c.is_ascii_digit())`. But we are not interested in the
character, but in the integer.

`char` has a method that can return the numeric value of a digit
character. It has the following signature. 

```
pub fn to_digit(self, radix: u32) -> Option<u32>
```

Notice that the return type is an `Option`, because if all you have is
a character, you are not sure how to turn it into a number. E.g. What
is the value of `Y`?

So the code we would like to run is something along the lines of

```
    #[test]
    fn parse_any_digit_as_number() {
        let input = "1230";
        let parser = map(
          any(|c: char| c.is_ascii_digit()), 
          |c: char| c.to_digit(10).unwrap_or(0));

        let actual = parser.parse(input);

        let expected = Ok((1, "230"));
        assert_eq!(actual, expected);
    }
```

During the mapping we are unwrapping the option with a default value of
zero.

## Map
With our goal clear, we still are following our strategy. So, let's first
create a struct.

```
pub struct Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    parser: P,
    map: F,
}
```

Woah, that signature has a lot of stuff in it. Let's break it done step by step.

### Lifetimes & Generics
First of all there is a lifetime ``a` parameter. This lifetime is needed by
`Parser` and keeps track of the lifetime of the input.

Next are generic parameters `I` and `O`. They are used by the other generic
parameters `P` and `F`.  Almost all of these are aliases.

`I` is restricted to `I: 'a`. It should live as long as our input. Together
with the chosen letter `I`, it probably stands for some form of input.

`P` is restricted to `P: Parser<'a, I> + Sized`. So `P` is a parser that
produces an `I`. For example our `any` parser is a produces an `char`.

`F` is restricted to `F: Fn(I) -> O + Sized`. Now it becomes clear would
`I` and `O` are. They are the input and output for the function `F`. So
`F` is a function that is used to map the result of parser `P`, which is
of type `I`, into type `O`.

Pfew, that was a lot to take in. But with an overview all the parts make
sense.  

### `PhantomData`
Unfortunatly, this will not compile. The compiler warns us of several
unused parameters. E.g.

```plain
82 | pub struct Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
   |                ^^ unused parameter
   |
   = help: consider removing `'a` or using a marker such as `std::marker::PhantomData`
```

The advice of removing the lifetime parameter is not an option for us.
We need it for our parser `P`. Maybe the other advice is viable.

The doccumentation of `PhantomData` reads

> Zero-sized type used to mark things that "act like" they own a T.
>
> Adding a PhantomData<T> field to your type tells the compiler that 
> your type acts as though it stores a value of type T, even though
> it doesn't really. This information is used when computing certain 
> safety properties.

That is precisely our case. What a good compiler our compiler is. So
we will follow our compilers advice and add a phantom field to our
struct.

```
    phantom: PhantomData<&'a I>,
```

Which satisfies the compiler.

## Impl Parser
With our struct out of the way, we should focus on the implementation.
What we want to achieve is

1. Try the parser.
2. Map the result with our function, keeping the rest of our input.

Besides the verbose type signatures this translates very well into Rust.

```
impl<'a, I, O, P, F> Parser<'a, O> for Map<'a, I, O, P, F> where I: 'a, P: Parser<'a, I> + Sized, F: Fn(I) -> O + Sized {
    fn parse(&self, input: &'a str) -> Result<(O, &'a str), ParseError> {
        let attempt = self.parser.parse(input);
        attempt.map(|(v, rest)|{ ((self.map)(v), rest)})
    }
}
```

## Constructor & Factory
Creating the constructor and factory is almost straight forward, albeit
verbose. Accept the parameters you need and create the `Map`. The only
snag is that `PhantomData`.

Luckily the way to assign a `PhantomData` is by doing just that.

## Summary
We have just created our first combinator! It takes a parser and transforms
its result into something different.

## Exercises
1. Implement the `Map` parser.
2. Write some tests to check your implementation.
