# `level`
In this chapter we get down to the brass tacks and implement the `level`
function of the `Interpreter`.

## Process
As we learned in the L-system section, interpretation of the system comes down
to repeatedly iterating the following

1. Take the current _word_
2. Replace each _symbol_ with it's production.

A word for is a vector of symbols. A symbol is anything that implements the
`Symbol` trait.

Knowing the production for each symbol is the job of the L-system, specifically
the rules of the L-system.

To start this process of, we need a jumping of point. Our `axiom`. Is exactly
what we need.

So it seems that we have everything in place to get started.

## `successors`
[Iterators][iterator] are ubiquitous in Rust. From the documentation

> If you've found yourself with a collection of some kind, and needed to perform
> an operation on the elements of said collection, you'll quickly run into
> 'iterators'. Iterators are heavily used in idiomatic Rust code, so it's worth
> becoming familiar with them.

Since we want to iterate our process, iterators should fit the bill. And Rust
doesn't disappoint.

In the standard library there is an iterator [`Successors`][successors] that is
perfect for our scenario. We only need to provide the first item and the
`succ` function that compute each item's successor

## first
The first item of our successors iterator it the axiom of our L-system. Because
we need to transfer ownership to the caller, we better clone the axiom before
iterating on it.

```
let start: Vec<T> = self.lsystem.axiom.iter().cloned().collect();
```

## `succ`
The `succ` function signature is `FnMut(&Vec<T>) -> Option<Vec<T>>`. It is
called with the our current word, and should return the successor.

```
|word|{
    let next = word.iter().flat_map(|symbol| self.lsystem.rules.apply(*symbol)).collect();
    Some(next)
})
```

Let's unpack that, on step at the time.

To access the individual symbols in our word we iterate over them with
`word.iter()`. Next we want to map each symbol to its production, hence the
`|symbol| self.lsystem.rules.apply(*symbol)`. But since each production is a
vector of symbols in itself, we use a [flat map][flat_map] instead of a regular
map. Since we want a `Vec<T>` at some point we `collect` the successor.

## Putting it together
All the moving parts of the implementation are in place, we just have to put
together.

```
pub fn level(&self, n : usize) -> Vec<T> {
    let start: Vec<T> = self.lsystem.axiom.iter().cloned().collect();
    successors(Some(start), |word|{
        let next = word.iter().flat_map(|symbol| self.lsystem.rules.apply(*symbol)).collect();
        Some(next)
    }).nth(n).unwrap() // there should always be an production
}
```

One final trick can be found at the end. It uses the `nth` function on
`Iterator`s to pick the word we want. Since we always produce an next word, the
unwrap can be safely done.

## Exercises
1. Implement the `level` function of our interpreter

[iterator]: https://doc.rust-lang.org/std/iter/index.html
[successors]: https://doc.rust-lang.org/std/iter/fn.successors.html
[flat_map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
