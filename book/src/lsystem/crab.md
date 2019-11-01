# Crab graphics
You might wonder how these image are being drawn. This chapter will explain the
connection between the words of the L-system and image being drawn.

## Ferris the Crab
![Ferris the Crab](/image/ferris.jpeg)
The artist is [@whoisaldeka](https://twitter.com/whoisaldeka).

We all know and love Ferris. But did you know that Ferris is a skilled and
attentive artist. Ferris is willing to lend their skill for our workshop. By
listening to our instructions they will draw an image for us.

Below are the command that Ferris responds to. Each command has a
corresponding symbol that Ferris knows, understands and responds to.

### Forward `F`
The forward command will make Ferris take a step forward, leaving a line in
their trail. The step size is determined by the `step` value of the
configuration.

### TurnLeft `-`
When the turn left command is issued Ferris will change heading accordingly. The
angle through which Ferris will turn is configured with the `angle` value of the
configuration.

### TurnRight `+`
Correspondingly, Ferris knows about how to turn right.

### Push `[`
Not only is Ferris a great artist, they have excellent memory. With the push
command Ferris will remember their position and heading so that they are able to
come back to it at a later time, i.e. when the corresponding pop command is
issued.

### Pop `]`
What would push be without a pop? Ferris agrees and will respond to the pop
command by remembering position and heading from the corresponding push command.
Push and pop commands should properly match. Just like the brackets used to
represents the commands.

### Any other symbol
Any other symbol besides the ones described above will be ignored by Ferris.
That does not mean that they aren't important. For they can help in constructing
more complex drawings.
