# L-System Material
This contains the material for the L-System workshop.

## Serving the book
The book is best served from a webserver. One option is to use
[`miniserve`][miniserve]. It can be installed with the following command

```sh
cargo +nigthly install miniserve
```

This requires the nightly channel for now which can be installed with 
`rustup update nightly`. Once installed you can serve the book by running 

```sh
miniserve book
```

And open [`localhost:8080/index.html`](http://localhost:8080/index.html)

[miniserve]: https://github.com/svenstaro/miniserve
