# Saddle Points

Welcome to Saddle Points on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Detect saddle points in a matrix.

So say you have a matrix like so:

```text
    1  2  3
  |---------
1 | 9  8  7
2 | 5  3  2     <--- saddle point at column 1, row 2, with value 5
3 | 6  6  7
```

It has a saddle point at column 1, row 2.

It's called a "saddle point" because it is greater than or equal to
every element in its row and less than or equal to every element in
its column.

A matrix may have zero or more saddle points.

Your code should be able to provide the (possibly empty) list of all the
saddle points for any given matrix.

The matrix can have a different number of rows and columns (Non square).

Note that you may find other definitions of matrix saddle points online,
but the tests for this exercise follow the above unambiguous definition.

## Rust Indices Start At 0

By convention, ordered sequences of values in Rust have their contents numbered
("indexed") starting from 0. This applies regardless of what the rest of the
exercise description in this README says, such as references to indices that
start at 1, so you will have to subtract 1 to translate those index numbers
to Rust index numbers.

## Efficiency Notice

This exercise uses a _vector of vectors_ to store the content of matrices. While
this exercise is designed to help students understand basic concepts about
vectors, such as indexing, and that nested data types are legal, _vector of
vectors_ is a suboptimal choice for high-performance matrix algebra and any
similar efficient processing of larger amounts of data.

The detailed explanation of this inefficiency is beyond the scope of this
exercise and this learning track in general. This aspect is known as
[cache locality](https://stackoverflow.com/questions/12065774/why-does-cache-locality-matter-for-array-performance)
and you can find a good introduction to it by clicking that link if you'd like
to learn more about details of a modern computer architecture.

## Source

### Created by

- @hekrause

### Contributed to by

- @bitfield
- @ClashTheBunny
- @coriolinus
- @cwhakes
- @efx
- @Emerentius
- @ErikSchierboom
- @michalfita
- @mtodor
- @murlakatamenka
- @petertseng
- @RiderSargent
- @rofrol
- @scarvalhojr
- @stringparser
- @workingjubilee
- @xakon
- @ZapAnton

### Based on

J Dalbey's Programming Practice problems - http://users.csc.calpoly.edu/~jdalbey/103/Projects/ProgrammingPractice.html