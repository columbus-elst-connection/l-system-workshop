# Basic Renderer Configuration

Our renderer instructions are very simple and abstract. For example, to draw a straight line we would use the `Forward` instruction. The `Forward` instruction doesn't itself say _how far_ to move. That is instead specified in a separate struct as the `step` field. Whenever we execute the `Forward` instruction, `step` tells us how far to move. Why would these be separate? The answer is that the language we use to describe the L-system has no way to maintain state. The Renderer implementation is responsible for maintaining the state of the current `step` and `angle`, but the initial values are specified in the `RendererConfig`.

The basic `RendererConfig` struct looks something like:

```rust
pub struct RendererConfig {
    pub step: u16,
    pub angle: u16,
}
```

We'll want a function that takes the configuration as a parameter and returns a new `Crab` struct that's ready to use.
