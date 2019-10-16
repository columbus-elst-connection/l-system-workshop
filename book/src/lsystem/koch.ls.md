# koch.ls
You may have noticed a different L-system in the `system` directory. I.e.
`koch.ls`. It is reproduced here for convenience.

```plain
config:
step = 5
angle = 60

rules:
axiom = F
F => F-F++F-F
```

Use this L-system in the `cargo run` command, and try to predict the answers.

## A picture is worth a thousand words
L-systems would still be worthwhile if the output is only a sequence of symbols.
But it really shines when these symbols are interpreted as a description of a
image.

Let's make that possible. Open the `src/main.rs`file. You don't need to
understand any of this, we will explain all of it during this workshop.

There are a few comments instructing to *uncomment* certain lines to use the
crab renderer, what ever that should be.
Do that now and then run the following command

```sh
cargo run -- -f system/koch.ls -n 4
```

## Exercises
1. Iterate the words for the `koch.ls` L-system by hand for a few values.
2. Run the same command with different number of iterations.
