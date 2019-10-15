# Use the framework
We have created a parser combinator framework that allows us to create a parser
that parses the L-systems by combining primitive parser into more elaborate
parsers.

In this chapter we will work towards that goal. It will be one of the most
challenging chapters in this book. You are asked to use your creative mind to
fill in the gaps.

## High level description
We will give a top down description of the structure of the L-system format.
This identifies goals to reach and helps with achieving

When seeing the L-system format from a mile away, it seems to consist of two
parts. First is the configuration section, which defines common settings for the
interpretation of the L-system.
Next is some number of blank lines followed by the actual rules of the system.

### Config
The _configuration_ section is started with a _header_, i.e. a literal `config` followed
by a colon and a newline.

Next are two key value pairs. A key value pair is a an literal key followed by
an equal sign and a certain value. The first key value pair is the _step_, which
assigns the _number_ of times the system needs to be iterated.
After that the _angle_ which determines the _number_ of degrees a turtle should
rotate.

### Rules
Just like the configuration section is the _rules_ section started with it's own
header. This time the header identifier is `rules`.

Next comes a the seed for the system, i.e the _axiom_ which is a key value pair
which should be at least 1 _symbol_.

Following that are many _rules_. Each _rule_ is described by a _symbol_ that
will be mapped onto its _production_, i.e. a sequence of at least 1 symbol.

## Exercises
Use the parser combinator framework to create a parser that can parser system
files. Make sure to test your parser building blocks.

Don't forget to alter the `parse` function to use the parser you created.
