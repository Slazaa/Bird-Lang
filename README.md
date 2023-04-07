# Bird
Hey, I'm Slazaa, and this is my programming language Bird!

## What is it ?
Bird is a general purpose, compiled programming language. For now the goal is 
to transpile to C, but once the it is bootstrapped in itself, my goal will be 
to use LLVM.

## What's the mindset ?
My actual mindset for now is to have a simple grammar, but I might think about
adding syntactic sugar in the future. I also want Bird to be highly perfomant.

## How does it handle memory ?
Bird memory management is handled manually. Though it will be using allocators,
so I could imagine a garbage collector allocator being made or something.

## Any inspirations from other languages ?
Yes! Bird is inspired from Rust and Zig mostly.

Rust through patterns, and the way that things like conditions can be
expressions, in fact, everything in Bird is an expression.

And Zig by it's compile time feature, and how it handles memory through
allocators that you can pass everywhere you need allocations. And also through
its build system, with a Bird file.

## What does it looks like ?
Here is a Hello World program!
```br 
box std = import("std");

box main = fn {
    std.println("Hello World!", .{ });
};
```
Earlier when I was talking about syntactic sugar, here is an example of
something I could add.
```
fn main {

};
```
Where it would translate to this.
```
box main = fn {

};
```
Also arrays can look a bit long.
```br 
Array.from(.{ 1, 2, 3 });
```
So maybe something like that could be nice.
```br 
[1, 2, 3];
```

## Can I start playing with it ?
Unfortunately no, for now only a Rust version of the parser is available. But
I'm working on making the C transpiler ready!
