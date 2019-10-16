# Renderer Configuration

Our renderer instructions are very simple and abstract. For example, to draw a straight line we would use the `Forward` instruction. The `Forward` instruction doesn't itself say _how far_ to move. That is instead specified in a separate struct as the `step` field. Whenever we execute the `Forward` instruction, `step` tells us how far to move. Why would these be separate? The answer is that the language we use to describe the L-system has no way to maintain state. We can provide instructions to _modify_ the state that we store, though. For example, we could have an instruction to multiply `step` by some global `step_multiplier`, and another instruction to divide it. This allows us to change the scale of our drawings, the line width, and things like that. The Renderer implementation is responsible for maintaining the state of the current `step`, `position`, `heading`, and `line_width`, but the initial values are specified in the `RendererConfig`. The `RendererConfig` also specifies the `pen_color`, `background_color`, and other things that will not be mutable in our implementation.

The full `RendererConfig` struct looks something like:

```rust
pub struct RendererConfig {
    pub starting_step: f64,
    pub step_multiplier: f64,
    pub starting_angle: f64,
    pub angle_multiplier: f64,
    pub starting_line_width: f64,
    pub line_width_multiplier: f64,
    pub background_color: String,
    pub pen_color: String,
}
```

`background_color` and `pen_color` are both `String`s that can be any of the [color names](https://docs.rs/turtle/1.0.0-rc.2/turtle/color/static.COLOR_NAMES.html) supported by the Turtle crate.
