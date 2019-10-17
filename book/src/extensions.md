# Why stop here
You have done something awesome! You have explored the wonderful world of
L-systems. You have created software to be able explore interesting aspects of
L-systems, parsers, interpreters and how they interact.

But known this is only a tip of the iceberg. There are a lot of things to
explore further. And this chapter is shining some light into the adventurous
world of exploration.

Don't take it as a travelers guide. We ourselves are as oblivious as the next
person when it comes to what you want to delve into. Feel free to follow your
own interest and amaze us with your thoughts and creations.

## More Rendering Instructions
We're able to create some awesome drawings, even with our basic set of rendering instructions. There's a lot of additional instructions that we could add in order to create increasingly complex images. Some of these instructions may also imply additional values in the `RenderConfig`.

- Instructions to increase and decrease the `step`
    - The amount to increase/decrease can be a configurable `step_multiplier` in the `RenderConfig`
- Instructions to increase and decrease the `angle`
    - The amount to increase/decrease can be a configurable `angle_multiplier` in the `RenderConfig`
- Instructions to increase and decrease the line width
    - The renderer config should include a starting value for the line width
    - The amount to increase/decrease can be a configurable `line_width_multiplier` in the `RenderConfig`
- Instructions to change the pen color
    - The renderer config should include a starting value for the pen color
    - How you change this could be simple or very complex. You could have separate instructions for each of a limited number of colors (e.g. `SetPenColorRed`), or you could create instructions for manipulating RGBA values independently. This could quickly get pretty complex, but it could also allow for some fancy coloring!

## Additional Renderer Configuration
Currently the parser for the `RenderConfig` only allows to define the step size and the angle. It may be good to add additional configuration items to support the new instructions suggested above.

Also, there's a lot of things you may want to configure more statically, like:

- Configure the starting position and heading of the Crab
- Configure the size of the canvas
- Configure the background and pen colors

## Make all length units relative

Our current implementation uses pixels as the unit for all distances, since that's what the turtle crate uses. This makes it sensitive to different resolutions. We could instead make these units relative to the size of the window, so that the images look good regardless of the size of the window or monitor.

## Parser Framework
Maybe you have discovered some recurring pattern while writing your parsers.
This is an ideal target for create your own combinators. Go back and examine
your code to see where some form of abstraction could be teased from it.

## Improved Interpreter
Currently the `Interpreter` returns the resulting word in one go, which requires storing the entire array of symbol productions in memory. You also have learned that words in L-systems can grow very quickly. This is a recipe for disaster.

Instead of returning the word in it's entirety, you could return an iterator over the word. This allows some flexibility how to iterate over the words.

One other way is to use a stack with the parts of words to progress combined with the level of iteration. Once the level reached zero, we can push the symbols out.

## Decouple the L-system and the Rendering

The L-system itself if just about symbol substitution. The symbols themselves don't have any intrinsic meaning. The fact that there's a `config:` section in our l-system file is less than ideal because it couples the interpreter and the renderer. Ideally, the two would be totally separate.

One idea to decouple these would be to write the interpreter and renderer as completely separate binaries. The interpreter would simply output a final sequence of symbols to stdout. The renderer would read a sequence of symbols from stdin and interpret them as rendering instructions. You could easily pipe the output of the interpreter into the renderer. This would make it really easy to try out different renderers for the same L-system!
