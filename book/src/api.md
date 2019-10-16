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
number of degrees the Crab will turn when we ask her to.

## RenderInstruction

## Symbol

## Rule

## LSystemRules

## LSystem
