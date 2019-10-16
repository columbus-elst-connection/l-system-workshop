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

## L-System Parser
Currently the parser for the `RenderConfig` only allows to define the step size
and the angle. One way to change how the L-system interacts is to change the
step size depending on the level of iteration.

Extend the L-system parser so that the step multiplication can be provided as
well. 

## Parser Framework
Maybe you have discovered some recurring pattern while writing your parsers.
This is an ideal target for create your own combinators. Go back and examine
your code to see where some form of abstraction could be teased from it.

## Improved Interpreter
Currently the `Interpreter` returns the resulting word in one go. You also have
learned that words in L-systems can grow very quickly. This is a recipe for
disaster.

Instead of returning the word in it's entirety, you could return an iterator
over the word. This allows some flexibility how to iterate over the words.

One other way is to use a stack with the parts of words to progress combined
with the level of iteration. Once the level reached zero, we can push the
symbols out.
