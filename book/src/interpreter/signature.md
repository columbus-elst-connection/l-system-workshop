# Signature
Let's think about the signature of the interpreter. We do this by coming up with
a code example that we would like to write. This will guide our thoughts towards
an API that is easy to work with. Below you find our dream code.

```
let interpreter = Interpreter::new(system);

let word = interpreter.level(4);

println!("{:?}", word)
```

We assume that we have a L-system bound to the variable `system`. It does not
really matter where we got the system from, just assume that we have it for now.

We would like to create a `Interpreter` for this system by calling the `new`
constructor.

Next we would like to ask the interpreter for the `word` that is produced by
iterating the L-system a number of times. This should produce a vector of
symbols.

## Interpreter Struct
Our dream code tells us a lot about the structure of our interpreter. Let's
flesh out some of the details to make our dreams come true.

```
use api::{LSystem, Symbol};

pub struct Interpreter<T> where T: Symbol {
    lsystem: LSystem<T>
}
```

We create a struct that will be the target for all of our dreams. It keeps track
of a `LSystem`. If you need a refresher about the api module, go read that
section.
