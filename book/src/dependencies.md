# Working with Dependencies
There are two different goals for this workshop. First is to allow you to play
with L-systems. Second is to allow you to code your own L-system, and then play
with that.

In order to facilitate that the code makes use of path dependencies. That way we
can easily transition from relying on provided code to relying on your own code.

## Mechanism
If you take a look at the `Cargo.toml` file you will notice the `[dependencies]`
table has a lot of path dependencies. E.g. we have a dependency on `parser`
which can be found at `examplar-parser`, according to the following snippet.

```toml
[dependencies]
parser = { path = "examplar-parser" }
```

### Your code
Every `examplar-*` library has a regular counter-part. I.e. besides
`examplar-parser` there is also `parser` library. This library has some skeletal
code. It is an ideal starting ground for working on your own code.

### Hooking up your code
Once you are comfortable with your code, it is time to hook it up to the main
executable. This way you can play with your own L-system.

This is done by changing the top-level `Cargo.toml`. Specifically the
`[dependencies]` table. For the `[dependencies]` you need to point to the correct
path. E.g. for the parser dependency the path should be `parser` if you want to
use your own code. 

### Enjoy your L-System
If everything works out, you should be able to enjoy your own L-system
framework.

