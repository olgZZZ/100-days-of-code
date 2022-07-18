# Largest Series Product

Welcome to Largest Series Product on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Given a string of digits, calculate the largest product for a contiguous
substring of digits of length n.

For example, for the input `'1027839564'`, the largest product for a
series of 3 digits is 270 (9 * 5 * 6), and the largest product for a
series of 5 digits is 7560 (7 * 8 * 3 * 9 * 5).

Note that these series are only required to occupy *adjacent positions*
in the input; the digits need not be *numerically consecutive*.

For the input `'73167176531330624919225119674426574742355349194934'`,
the largest product for a series of 6 digits is 23520.

These iterators may be useful, depending on your approach

- [Windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
- [Product](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)

## Source

### Created by

- @IanWhitney

### Contributed to by

- @coriolinus
- @cwhakes
- @eddyp
- @efx
- @ErikSchierboom
- @lutostag
- @mkantor
- @nfiles
- @petertseng
- @rofrol
- @stringparser
- @xakon
- @ZapAnton

### Based on

A variation on Problem 8 at Project Euler - http://projecteuler.net/problem=8