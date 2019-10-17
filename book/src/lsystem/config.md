# config
While playing with different number of iterations for the Koch L-system. You
probably noticed that sometimes the image would be drawn outside the borders of
the screen.

For example, the command 

```
cargo run -- -f system/koch.ls -n 5
```

Will result in a picture similar to the one below.

![The Koch curve drawn outside the boundary](/image/koch-cutoff.png)

The `config` section in `koch.ls` will be enable us to remedy that.

## step
The config section has a number of key value pairs. One of them is `step`. This
controls how big the line segment is whenever line segment is drawn. When you
change it to `2` and run the the command above, we will capture the Koch curve
inside the window.

![The Koch curve in full glory](/image/koch.png)

## angle
There is an other option you can configure. I.e. `angle`. This determines the
number of degrees through which a turn is made. Change the configuration to have
a step of `30` and an angle of `80`. Next run the L-system for 4 iterations. 

![A pointier Koch curve](/image/pointy-koch.png)

## Exercises
1. Play with the configuration to produce a variety of different images.
