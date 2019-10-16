# Turtle (Crab) Rendering

Ok, so we have a Crab that's brandishing a pen, and we've got everything initialized and configured. So how do we actually work with this thing?
The `crab` API is pretty simple and straightforward. It's pretty easy to implement each individual rendering instruction. The complexity of how multiple instructions are combined to draw interesting shapes will be left to the L-systems.


### Forward

The `Forward` instruction moves the crab forward by the `step`, drawing a straight line along the way. Implementing this is quite simple. Just call the [`Turtle::forward`](https://docs.rs/turtle/1.0.0-rc.2/turtle/struct.Turtle.html#method.forward) function, passing in the current value for `step`.


### Rotation

`RotateLeft`, `RotateRight` rotate the crab in place. "Right" means clockwise, and "left" means counter-clockwise. Keep in mind that we're using degrees instead of radians (although, this is a fairly arbitrary decision and it's also reasonable for an l-system to use radians).
