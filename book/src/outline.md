# Outline
There are various ways of learning and we want to enable you to choose your path
through the material. Below you can find the main sections that we will address
during this workshop.

It is possible to progress through the sections in any order you want. We have
setup things in such a way that you can rely on our code so long as yours is not
ready.

## L-systems
This section teaches you what [L-systems][l-system] are, trains you how to
create them, and allows you to play with them.

Describing a L-system is one thing. Teaching a computer how to work with
L-systems is another. The next sections are major parts in the software that
allows you to explore L-systems.

## Parser
A [parser][] is a

> software component that takes input data (frequently text) and builds a data
> structure – often some kind of parse tree, abstract syntax tree or other
> hierarchical structure, giving a structural representation of the input while
> checking for correct syntax. 

This section focuses on parsing a L-system. We do this by creating our own
[Parser Combinator framework][parser combinator] that we use to read an L-system
and turn it into a data structure that we can use.

## Interpreter
Once you have defined an L-system, you probably would like to do something with
it. The [interpreter][] helps with this goal. It diligently executes the
instructions described by the L-system informing interested parties what has
happened.

The interpreter is the workhorse of our program. Although it is the centerpiece
of our software, it is a quite simple machine.

## Renderer
An L-system without a "picture" is as a Christmas tree without decorations. It
still is instructive, but is sure isn't as much fun.

In this section we explore various way of make a "picture" for our L-system.
There are various ways of turning the L-system into meaningful artifact. Making
an image is surely not the only one.


[l-system]: https://en.wikipedia.org/wiki/L-system
[parser]: https://en.wikipedia.org/wiki/Parsing
[parser combinator]: https://en.wikipedia.org/wiki/Parser_combinator
[interpreter]: https://en.wikipedia.org/wiki/Interpreter_(computing)
