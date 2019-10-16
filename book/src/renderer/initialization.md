# Renderer Initialization

Each operating system provides its own APIs for drawing images to the screen, and these APIs are sometimes fairly complex. Typically, there's a need to create a window, which you can then draw into. The APIs also provide ways to get input events from the user's mouse and keyboard. We want our L-systems renderer to run on a variety of platforms, and so we must program to the lowest common denominator of all the platforms we wish to support. The first function declared on the `Renderer` trait is `fn global_init()`.

The `global_init` function is the very first thing that's called when our application starts up. We'll use this function to create a window that we can draw into. Windowing and event handling apis can be relatively complex, and every platform has its own quirks. It's common for the operating system to require that window creation is done very early in the application lifecycle, so we'll always call this function right away in our application's main function. It's important that we call this function _before_ we do things like parsing arguments or writing to stdout and stderr.

## Implementation

Our implementation of `global_init` is pretty easy, since the turtle crate handles all the cross platform complexities for us. We just need to call the [`turtle::start()`](https://docs.rs/turtle/1.0.0-rc.2/turtle/fn.start.html) function.
