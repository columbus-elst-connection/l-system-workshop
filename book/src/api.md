# `api` Module

In this section we describe the api that is provided that forms the tapestries
onto we weave our application.

It is good to keep in mind that we are working toward a Crab renderer. I.e. a
turtle graphics capable ferris that responds to our commands.

## RenderConfig
```
#[derive(Debug, PartialEq)]
pub struct RenderConfig {
    pub step: u16,
    pub angle: u16,
}
```

The `RenderConfig` is used to configure the Crab renderer. `step` refers to the
distance the Crab will travel when stepping forward. `angle` refers to the
number of degrees the Crab will turn when we ask her to. This is what you'll parse from the l-system file.

## RenderInstruction

We define this enum just to give us a simple way to represent what we want our Renderer to do. Our basic definition looks like this:

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum RendererInstruction {
    Forward,
    RotateLeft,
    RotateRight,
    Push,
    Pop,
    NoOp,
}
```

This is a fairly minimal instruction set, but you can still do a lot with it. More full-featured L-system implementations may have [more instructions](./extensions.md), but we'll start simple for now.


## Symbol

The heart of any L-system is Symbols. It is, afterall, a symbol substitution language. `Symbol` is defined as a trait with a few simple properties. First, it must implement `Eq` and `Hash` since we will need to compare them and potentially store them in a hashmap. Secondly, it must implement `Copy` to facilitate easily producing `Symbol`s based on the rules in our L-system. Lastly, we want it to also implement `Display` so that we can produce a nicer output when debugging. For this workshop we'll use `char`, since it already implements all of those traits.

A `Symbol` must also implement the function `fn to_rendering_instruction(&self) -> RendererInstruction;`. This function maps each `Symbol` to an instruction for our renderer to execute. One important bit to point out is that our `RendererInstruction` enum includes a `NoOp` instruction, which instructs our Renderer to do nothing. We define this instruction so that it's clear when a `Symbol` does not map to any particular `RendererInstruction`. There are way more valid `char` values than there are `RendererInstruction` variants, so we need a clear way to represent the case when a `Symbol` is only used for matching substitutions.

For our workshop, the mapping between `char` values and rendering instructions will be hard coded as part of the `Symbol::to_rendering_instruction` function. This mapping follows a fairly common convention, but other implementations will handle this differently. Some implementations even allow you do define this mapping yourself, but we'll leave all that for [extra credit](./extenstions.md).

## Rule

The L-system file will contain a `rules:` section that has one or more rules for symbol replacement. In our API we'll represent each rule as a struct containing the `Symbol` to match and a `Vec<Symbol>` of replacements to produce. For example, the rule `F => F+F--F+F` would be represented by:

```rust
Rule {
    match_input: 'F',
    productions: vec!['F', '+', 'F', '-', '-', 'F', '+', 'F'],
}
```

## LSystemRules

Once we have all the `Rule`s parsed, we'll represent the complete set of rules as a `HashMap<char, Vec<char>>` where the keys are the `Symbol`s to match and the values are the productions for that symbol. This just facilitates easy lookups.

## LSystem

This struct is just a holder for the complete set of data that's been parsed from an l-system file. This represents almost everything we need in order to render the final image. The only additional piece we'll need is the number of iterations, but we pass that as a command line argument.
