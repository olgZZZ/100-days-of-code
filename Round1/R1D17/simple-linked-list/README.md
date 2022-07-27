# Simple Linked List

Welcome to Simple Linked List on Exercism's Rust Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Write a simple linked list implementation that uses Elements and a List.

The linked list is a fundamental data structure in computer science,
often used in the implementation of other data structures. They're
pervasive in functional programming languages, such as Clojure, Erlang,
or Haskell, but far less common in imperative languages such as Ruby or
Python.

The simplest kind of linked list is a singly linked list. Each element in the
list contains data and a "next" field pointing to the next element in the list
of elements.

This variant of linked lists is often used to represent sequences or
push-down stacks (also called a LIFO stack; Last In, First Out).

As a first take, lets create a singly linked list to contain the range (1..10),
and provide functions to reverse a linked list and convert to and from arrays.

When implementing this in a language with built-in linked lists,
implement your own abstract data type.

Do not implement the struct `SimpleLinkedList` as a wrapper around a `Vec`. Instead, allocate nodes on the heap.  
This might be implemented as:
```
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
```
The `head` field points to the first element (Node) of this linked list.  
This implementation also requires a struct `Node` with the following fields:
```
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```
`data` contains the stored data, and `next` points to the following node (if available) or None.  

## Why `Option<Box<Node<T>>>` and not just `Option<Node<T>>`?
Try it on your own. You will get the following error.

```
| struct Node<T>
| ^^^^^^^^^^^^^^ recursive type has infinite size
...
|     next: Option<Node<T>>,
|     --------------------- recursive without indirection
 ```

 The problem is that at compile time the size of next must be known.
 Since `next` is recursive ("a node has a node has a node..."), the compiler does not know how much memory is to be allocated.
 In contrast, [Box](https://doc.rust-lang.org/std/boxed/) is a heap pointer with a defined size.

## Source

### Created by

- @kedeggel

### Contributed to by

- @coriolinus
- @cwhakes
- @efx
- @ErikSchierboom
- @lutostag
- @ocstl
- @petertseng
- @rofrol
- @stringparser
- @tejasbubane
- @treble37
- @xakon
- @ZapAnton

### Based on

Inspired by 'Data Structures and Algorithms with Object-Oriented Design Patterns in Ruby', singly linked-lists. - https://web.archive.org/web/20160731005714/http://brpreiss.com/books/opus8/html/page96.html