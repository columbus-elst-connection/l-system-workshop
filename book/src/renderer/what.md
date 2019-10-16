# Renderer

A Renderer takes the sequence of productions from the L-system and displays them in some way. We've included an example renderer that simply prints each symbol to stdout. This is really useful for debugging, but it's not very pretty to look at.

In this workshop, we're going to create a new Renderer implementation that uses turtle graphics to render the productions from our l-system as two dimensional line drawings. Turtle graphics is an approach that uses an API that's conceptually based on a _pen_ that is controlled by giving it a sequence of instructions. For example, `go forward 2 steps, turn right, go forward 1 step, turn right, go forward 2 steps`. Of course a pen is an inanimate object and they're terrible at following instructions, so traditionally a turtle is in charge of the pen, though in our case it will be a crab instead. The crab will faithfully carry our each of the instructions in order to create our image.

## Renderer API

To start with, our renderer trait looks like this:

```rust
pub trait Renderer {
    /// Called at the very beginning of the entrypoint to our program so that the renderer can create a window if needed
    fn global_init() where Self: Sized {}

    /// Render the given symbol. This will get called repeatedly in order to create our images
    fn render(&mut self, symbol: impl Symbol);

    /// Signals that the final instruction has been given and we can clean up and do any finalization that's required
    fn finish(&mut self) {}
}
```

In the next sections, we'll go over each of the `Renderer` functions in more detail.

