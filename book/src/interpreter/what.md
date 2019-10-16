# What is an Interpreter
One could view an L-system as a kind of programming language. It describes in a
declarative way how to produce a series of symbols. What we will do in this
section is write an interpreter.

An [interpreter][wikipedia:interpreter] is

> is a computer program that directly executes instructions written in a
> programming or scripting language, without requiring them previously to have
> been compiled into a machine language program

In our case the instructions is the L-system, as parsed by our parser. Execution
is producing the series of symbols for a certain level.


[wikipedia:interpreter]: https://en.wikipedia.org/wiki/Interpreter_(computing)
