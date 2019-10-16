# Renderer

We're going to use turtle graphics to render our l-system. Turtle graphics is an approach that uses an API that's conceptually based on a _pen_ that is controlled by giving it a sequence of instructions. For example, `go forward 2 steps, turn right, go forward 1 step, turn right, go forward 2 steps`. Of course a pen is an inanimate object and they're terrible at following instructions, so traditionally a turtle is in charge of the pen, though in our case it will be a crab instead. The crab will faithfully carry our each of the instructions in order to create our image.

## Renderer API

To start with, our renderer trait looks like this:

```rust
pub trait Renderer {
    /// Called at the very beginning of the entrypoint to our program so that the renderer can create a window if needed
    fn global_init() where Self: Sized {}

    /// Creates a new instance of the renderer with the given configuration
    fn new(renderer_config: RendererConfig) -> Self;

    /// Render the given instruction. This will get called repeatedly in order to create our images
    fn render(&mut self, instruction: RendererInstruction);

    /// Signals that the final instruction has been given and we can clean up and do any finalization that's required
    fn finish(&mut self) {}
}
```

In the next sections, we'll go over each of these functions in more detail.



After `global_init` is done, the application will focus on parsing it's arguments and the l-system. Parsing the l-system file successfully will result in a `RendererConfig` struct, which will be passed to `Renderer::new`. It's here that we can initialize things like the background color, the starting line width, and other such things. After this is done, we should be ready to execute render instructions to create our image.

The `render` function is where the magic happens. `RendererInstruction` is an enum with a variant for each instruction. Each time it's invoked, we'll use pattern matching to determine what to do. This would look something like the following:

```rust
fn render(&mut self, instruction: RendererInstruction) {
    match instruction {
        RendererInstruction::Forward => self.forward(),
        RendererInstruction::RotateLeft => self.rotate_left(),
        RendererInstruction::RotateRight => self.rotate_right(),
        RendererInstruction::NoOp => { /* no-op */ },
        /* .... remainder omitted for brevity */
    }
}
```

Since this pattern matching isn't particularly interesting, we'll update our `Renderer` trait to provide the implementation on the trait itself, which will just delegate to named functions for each instruction. We'll add a function to the `Renderer` trait for each instruction so that the code will be more readable. So instead of implementing `render` directly, we'll just implemet each of the specific functions like `forward`, `rotate_left`, `rotate_right`, `push`, `pop`, etc. So now our `Renderer` trait will look more like this:

```rust
pub trait Renderer {
    fn global_init() where Self: Sized {}

    fn new(renderer_config: RendererConfig) -> Self;

    fn render(&mut self, instruction: RendererInstruction) {
        match instruction {
            RendererInstruction::Push => self.push(),
            RendererInstruction::Pop => self.pop(),
            RendererInstruction::Forward => self.forward(),
            RendererInstruction::NoOp => { /* no-op */ },
            /* .... remainder omitted for brevity */
        }
    }

    fn forward(&mut self) {}
    fn push(&mut self) {}
    fn pop(&mut self) {}
    /* .... remainder of functions omitted for brevity */

    fn finish(&mut self) {}
}
```


