# Plant
We are going to write our own L-system. In this chapter you will go through the
motions. The next chapter will explain in more detail how to rules are
interpreted. 

## plant.ls
Create a file in the `system` directory called: `plant.ls`. This will will
contain a description of our L-system.

## config
Begin with a `config` header. As you know this starts the configuration section
where you can configure the generation of the image. Configure the step to be
`5` and the angle to be `25`.

```plain
config:
step = 5
angle = 25
```

## rules
Next comes the rules section which is started with a `rules` header. The first
thing in a rules section should be the _axiom_. Make this to be `--X`.
The axiom is followed by a number of rules. For this L-system there are two. One
rule that tells us to transform `F` into `FF`. The other rule tells us to
transform `X` into `F-[[X]+X]+F[+FX]-X`.

```plain
rules:
axiom = --X
X => F-[[X]+X]+F[+FX]-X
F => FF
```

## Execution
With the `plant.ls` file in place, we can admire our work by executing the
command

```sh
cargo run -- -f system/plant.ls -n 5
```

![A delicate plant](/image/plant.png)

## Exercises
1. Create the `plant.ls` file as described in this chapter.
2. Play with it's configuration.
