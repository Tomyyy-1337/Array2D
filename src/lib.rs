//! This crate provides a macro to create a 2D array struct with the given width and height on the stack.
//! The data is stored in a single stack-allocated array to improve cache locality and reduce overhead from indirection and heap allocation.
//! 
//! # Example
//! ``` rust
//! use array2d::create_array2d;
//! create_array2d!(3, 4);
//! ```
//! 
//! This will create a struct named `Array2D_3_4` with a width of 3 and a height of 4.

pub extern crate array2d_derive;
pub use array2d_derive::create_array2d;