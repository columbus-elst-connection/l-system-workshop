# Exposé
Below you find a few images created from L-systems. It is your job to try to
recreate them. This way you get a feeling for how L-systems can be used to
recreate images.

before you start, Take a look at the exercises. They might provide a hint.

## Tree
![A tree](/image/tree.png)


## Perspective
![A road lined with trees](/image/perspective.png)

## Square
![A square](/image/square.png)

## Sqoch
![Sqoch](/image/sqoch.png)

## Koch Island
![Koch Island](/image/koch-island.png)

## Hilbert Curve
![Hilbert Curve](/image/hilbert.png)

## Exercises
1. What image corresponds with the L-system below. Try to make an educated guess
   before verifying your claim.
   
```plain
config:
step = 100
angle = 90

rules:
axiom = Q
Q => A+A
A => S+S
S => F
```

2. Try to create L-systems that recreate the images in this chapter.
