# Keyboard

This is a library that contains a generic representation of a keyboard. It is intended to be used for implementing keyboards on embedded devices, but it doesn't need to be run in an embedded environment. 

## How to use

### Overview

This lib provides a `Keyboard` struct which needs an instance of a key map, and an instance of someting that implements the `Matrix` trait. This trait provides a way to get which locations on the key map are active at any given time. 

In an embedded context, you'll likely want to implement the `Matrix` trait on some representation of a GPIO matrix. 

Calling `.poll()` on the `Keyboard` will evaluate the state of the matrix, and add any detected keypresses to an internal queue. Afterwards the keypresses can be retrieved from the queue.

### Keymaps

todo...

## Future work

- add helper functions/macros to more easily build keymaps. 
