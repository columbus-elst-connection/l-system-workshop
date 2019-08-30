# Combinators
We havee implemented a few parser. Each followed the same pattern.

1. Choose a datatype that allows the parser to do it's job.
2. Implement `Parser` for it.
3. Create a _constructor_.
4. Create a factory.

This pattern suffices, but it get's tedious. Instead we would rather
use the power of _abstraction_. Rather than to depend on the manual
work to create a parser for each situation, we want to have some 
primitive parser that we can combine into a more elaborate parser.

This chapter introduces you to the concept and some considerations
we will meet along the way.

## What is a combinator
In our case a combinator is a _higher order_ parser. It takes one or
more parsers and *combines* them into a new parser.

Let's stare into the future together, in order to better understand
combinators.

Imagine that we have a parser that could parse a single rule of a 
L-system. How we acquired this parser is of no concern. Just that
we can use it.

Now as you know, L-systems often have many rules. So we would like
a parser that could parse many rules. We could use the rule parser
and manually try to parse a rule, and when it succeed try for the
next rule.

This would certainly work! But it would not help in the situation
where we have a parser to parse a single whitespace and would want
a parser to parse a many whitespaces. This is a very similar 
situation to the rule parser, the only difference being the kind
of parser you want many of.

Instead we are going to write a *combinator*. This is a parser that
accepts as an argument one or more parsers, in our example the rule
parser or the whitespace parser, and returns a parser that combines
them in an interesting way. In our example that would be the many
combinator.

## Combinator Pattern
What we have described for parser is an example of a more general
pattern. I.e. the [combinator pattern][pattern]. It says

> complex structures are built by defining a small set of very simple 
> 'primitives', and a set of 'combinators' for combining them into 
> more complicated structures. 

If you squint hard you can find our explanation of the phenomenon
for parser in the description.

So the next few chapters will concentrate on the descriptio of
various combinators.

[pattern]: https://wiki.haskell.org/Combinator_pattern 