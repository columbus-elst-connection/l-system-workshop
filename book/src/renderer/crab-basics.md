# Turtle (Crab) Rendering

Ok, so we have a `Crab` that's brandishing a pen, and we've got everything initialized and configured. So how do we actually work with this thing?
The `Turtle` API is pretty simple and straightforward. It's easy to implement each individual rendering instruction. The complexity of how multiple instructions are combined to draw interesting shapes will be left to the L-systems.

The `render` function is where the magic happens. This function will be invoked many times, once for each `Symbol` that's produced by the L-system, based on the axiom and number of iterations. The first thing we need to do is to convert the `Symbol` into a  `RendererInstruction`. Then we can use pattern matching to determine what to do. This would look something like the following:

```rust
fn render(&mut self, symbol: impl Symbol) {
    match symbol.to_rendering_instruction() {
        RendererInstruction::Forward => self.forward(),
        RendererInstruction::RotateLeft => self.rotate_left(),
        RendererInstruction::RotateRight => self.rotate_right(),
        RendererInstruction::NoOp => { /* no-op just means that the Symbol didn't map to any particular rendering instruction */ },
        /* .... remainder omitted for brevity */
    }
}
```

Since this pattern matching isn't particularly interesting, we'll update our `Renderer` trait to provide the implementation on the trait itself, which will just delegate to named functions for each instruction. We'll add a function to the `Renderer` trait for each instruction so that the code will be more readable. So instead of implementing `render` directly, we'll just implemet each of the specific functions like `forward`, `rotate_left`, `rotate_right`, `push`, `pop`, etc. So now our `Renderer` trait will look more like this:

```rust
pub trait Renderer {
    fn global_init() where Self: Sized {}

    fn render(&mut self, instruction: RendererInstruction) {
        match instruction {
            RendererInstruction::Forward => self.forward(),
            RendererInstruction::RotateLeft => self.rotate_left(),
            RendererInstruction::RotateRight => self.rotate_right(),
            RendererInstruction::Push => self.push(),
            RendererInstruction::Pop => self.pop(),
            RendererInstruction::NoOp => { /* no-op */ },
        }
    }

    fn forward(&mut self) {}
    fn push(&mut self) {}
    /* .... remainder of functions omitted for brevity */

    fn finish(&mut self) {}
}

impl Renderer for Crab {
    fn forward(&mut self) { /* move forward */ }
    fn rotate_left(&mut self) { /* ... */ }
    fn forward(&mut self) { /* ... */ }
}
```

### Forward

The `Forward` instruction moves the crab forward by the `step`, drawing a straight line along the way. Implementing this is quite simple. Just call the [`Turtle::forward`](https://docs.rs/turtle/1.0.0-rc.2/turtle/struct.Turtle.html#method.forward) function, passing in the current value for `step`.


### Rotation

`RotateLeft`, `RotateRight` rotate the crab in place. "Right" means clockwise, and "left" means counter-clockwise. Keep in mind that we're using degrees instead of radians (although, this is a fairly arbitrary decision and it's also reasonable for an l-system to use radians).
