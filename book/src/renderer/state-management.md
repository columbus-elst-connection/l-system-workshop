# Renderer State Management

You'll notice that our `RendererInstruction` enum includes operations for `Push` and `Pop`. These instructions are important for rendering more complex shapes because they allow us to "save" the state of the renderer and go back to it later. As you probably guessed, the renderer states are "saved" by pushing them onto a stack using the `Push` instruction. We can then re-load that saved state by `Pop`ing from the stack.

### State

Here's an example of how we can represent this state:

```
struct State {
    position: turtle::Point,
    heading: turtle::Angle,
    step: u16,
}
```

Our `Crab` can use a `Vec<State>` as a stack. Whenever a `Push` instruction is executed, we capture the current state and push it onto the stack. Whenever a `Pop` instrution is executed, we'll pop a State from the stack and set the location, heading, etc. of our crab. Simple! Using `Push` and `Pop` instructions is what allows for some really complex and interesting images.

