//! This crate provides a macro to create a 2D array struct with the given width and height on the stack.
//! The data is stored in a single stack-allocated array to improve cache locality and reduce overhead from indirection and heap allocation.
//! 
//! # Example
//! ``` rust
//! use array2d::create_array2d;
//! create_array2d!(3, 4);
//! 
//! let mut array_2d = Array2D_3_4::new(1u64);
//! array_2d[1][1] = 2;
//! assert_eq!(array_2d[1][1], 2);
//! ```
//! 
//! This will create a struct named `Array2D_3_4` with a width of 3 and a height of 4.
pub extern crate array2d_derive;

/// Create a 2D array struct with the given width and height.
/// 
/// # Examples
/// ```
/// # use array2d::create_array2d;
/// create_array2d!(3, 4); // creates a struct named Array2D_3_4
/// let array = Array2D_3_4::new(6u64); // creates a new 3x4 array with all elements set to 6
/// array.iter().for_each(|&x| assert_eq!(x, 6));
/// ```
/// 
/// ``` 
/// # use array2d::create_array2d;
/// create_array2d!(3, 4);
/// let mut array = Array2D_3_4::default();
/// array.set_unchecked(1, 1, 1);
/// array[2][2] = 2;
/// 
/// assert_eq!(array.get_unchecked(1, 1), &1);
/// assert_eq!(array.get_unchecked(2, 2), &2);
/// assert_eq!(array.get_unchecked(0, 0), &0);
/// assert_eq!(array.get(1, 1), Some(&1));
/// assert_eq!(array[2][2], 2);
/// assert_eq!(array.get_mut(2, 2), Some(&mut 2));
/// ```
pub use array2d_derive::create_array2d;
