# Command Line Interface

The entrypoint to our program is a good place to start, so that we can get a sense for how all the pieces will fit together. The interface we're looking for is something like this:

```
l-system -f <l-system-file> -a <axiom> -n <iterations>
```

We need three main pieces in order to render an l-system. First is a file containing the description of our l-system. This will have the rules for token replacements, and it will also contain some starting configuration for our renderer. Next is the _axiom_, which is a string of starting tokens to feed into our l-system. Lastly we have the number of iterations, which is the number of times that we should apply the token substitution rules.

The `main` function should parse these arguments, parse the l-system described in the file, then get the final productions and pass them to the renderer as individual instructions. Once that is complete, it should call finalize on the renderer.
