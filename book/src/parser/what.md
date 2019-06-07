# What is a Parser?
A [parser][] is a

>  software component that takes input data (frequently text) and builds a data
>  structure – often some kind of parse tree, abstract syntax tree or other
>  hierarchical structure, giving a structural representation of the input while
>  checking for correct syntax.

In our case we want to teach the computer to understand `.ls` files. This is our
_input data_. Although this input is convenient for us, it is just a sequences of
bytes to a computer.

What we want is an L-system that we can manipulate. That is our _data
structure_. Parsing is what brings us from our input data to our L-system.

## Parser framework
We will be building our own Parser framework. I.e. we will build the
infrastructure that allows us to create a custom parser. We will use our
framework to create a specific parser that reads in our `.ls` files and create
the L-system for us.

## Parser Combinator
Our framework will be based on the concept of Parser Combinators. A Parser
Combinator is a

>  higher-order function that accepts several parsers as input and returns a new
> parser as its output. In this context, a parser is a function accepting strings
> as input and returning some structure as output, typically a parse tree or a set
> of indices representing locations in the string where parsing stopped
> successfully.

If this does not really makes sense to you. Keep reading and soon you will be an
expert Parser Combinatorist.

[parser]: https://en.wikipedia.org/wiki/Parsing
[combinator]: https://en.wikipedia.org/wiki/Parser_combinator
