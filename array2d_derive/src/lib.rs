extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitInt, Token, parse::{Parse, ParseStream}};

struct Array2DInput {
    width: LitInt,
    height: LitInt,
}

impl Parse for Array2DInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let width: LitInt = input.parse()?;
        input.parse::<Token![,]>()?;
        let height: LitInt = input.parse()?;
        Ok(Array2DInput { width, height })
    }
}

/// Creates a 2D array struct with the given width and height on the stack using a single array.
/// 
/// # Parameters
/// - width: usize - The width of the 2D array.
/// - height: usize - The height of the 2D array.
#[proc_macro]
pub fn create_array2d(input: TokenStream) -> TokenStream {
    let Array2DInput { width, height } = parse_macro_input!(input as Array2DInput);

    let width = width.base10_parse::<usize>().unwrap();
    let height = height.base10_parse::<usize>().unwrap();
    let len = width * height;
    let struct_name = format_ident!("Array2D_{}_{}", width, height);

    let expanded = quote! {
        /// 2D array with fixed width and height.
        /// The data is stored in a single stack-allocated array to improve cache locality and prevent overhead from indirection and heap allocation.
        /// 
        /// # Example
        /// ``` rust
        /// use array2d::create_array2d;
        /// create_array2d!(3, 4);
        /// 
        /// fn main() {
        ///     let array = Array2D_3_4::default(0u16);
        /// }
        pub struct #struct_name<T> {
            pub data: [T; #len],
        }
        
        impl<T> #struct_name<T> {
            /// Returns the width of the 2D array.
            pub fn width(&self) -> usize {
                #width
            }

            /// Returns the height of the 2D array.
            pub fn height(&self) -> usize {
                #height
            }

            /// Returns the total number of elements in the 2D array.
            pub fn len(&self) -> usize {
                #len
            }

            /// Creates a new 2D array with the given data. 
            /// 
            /// # Parameters
            /// - data: [T; #len] - The data to initialize the 2D array with. 
            /// 
            /// # Returns
            /// - Self - The new 2D array.
            pub fn from_raw(data: [T; #len]) -> Self {
                #struct_name {
                    data,
                }
            }

            /// Returns a reference to the raw data of the 2D array.
            /// 
            /// # Returns
            /// - &[T; #len] - A reference to the raw data of the 2D array.
            pub fn get_raw(&self) -> &[T; #len] {
                &self.data
            }

            /// Creates a new 2D array with the given default value.
            /// 
            /// # Parameters
            /// - default: T - The default value to initialize the 2D array with.
            /// 
            /// # Returns
            /// - Self - The new 2D array.
            pub fn new(default: T) -> Self
            where
                T: Copy,
            {
                #struct_name {
                    data: [default; #len],
                }
            }

            /// Returns the element at the given x and y coordinates or None if the coordinates are out of bounds.
            /// 
            /// # Parameters
            /// - x: usize - The x coordinate.
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - Option<&T> - A reference to the element at the given coordinates or None if the coordinates are out of bounds.
            /// 
            /// # Example
            pub fn get(&self, x: usize, y: usize) -> Option<&T> {
                if x < #width && y < #height {
                    Some(&self.data[y * #width + x])
                } else {
                    None
                }
            }

            /// Returns a mutable reference to the element at the given x and y coordinates or None if the coordinates are out of bounds.
            /// 
            /// # Parameters
            /// - x: usize - The x coordinate.
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - Option<&mut T> - A mutable reference to the element at the given coordinates or None if the coordinates are out of bounds.
            pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
                if x < #width && y < #height {
                    Some(&mut self.data[y * #width + x])
                } else {
                    None
                }
            }

            /// Sets the element at the given x and y coordinates to the given value and returns true if the coordinates are in bounds.
            /// 
            /// # Parameters
            /// - x: usize - The x coordinate.
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - bool - True if the coordinates are in bounds, false otherwise.
            pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
                if x < #width && y < #height {
                    self.data[y * #width + x] = value;
                    return true;
                }
                false
            }

            /// Sets the element at the given x and y coordinates to the given value without checking bounds.
            /// 
            /// # Safety
            /// This method can panic or modify wrong cells if the coordinates are out of bounds.
            /// 
            /// # Parameters
            /// - x: usize - The x coordinate.
            /// - y: usize - The y coordinate.
            pub fn set_unchecked(&mut self, x: usize, y: usize, value: T) {
                self.data[y * #width + x] = value;
            }

            /// Returns the element at the given x and y coordinates without checking bounds.
            /// 
            /// # Safety
            /// This method can panic or modify wrong cells if the coordinates are out of bounds.
            /// 
            /// # Parameters
            /// - x: usize - The x coordinate.
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - &T - A reference to the element at the given coordinates.
            pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
                &self.data[y * #width + x]
            }

            /// Returns a mutable reference to the element at the given x and y coordinates without checking bounds.
            /// 
            /// # Safety
            /// This method can panic or modify wrong cells if the coordinates are out of bounds.
            /// 
            /// # Parameters
            /// - x: usize - The x coordinate.
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - &mut T - A mutable reference to the element at the given coordinates.
            pub fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
                &mut self.data[y * #width + x]
            }

            /// Iterates over the 2D array row by row.
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                self.data.iter()
            }

            /// Iterates over the 2D array row by row mutably.
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
                self.data.iter_mut()
            }

            /// Iterates over the row at the given y coordinate.
            /// 
            /// # Parameters
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - impl Iterator<Item = &T> - An iterator over the row at the given y coordinate.
            pub fn iter_row(&self, y: usize) -> impl Iterator<Item = &T> {
                self.data[y * #width..(y + 1) * #width].iter()
            }

            /// Iterates over the row at the given y coordinate mutably.
            /// 
            /// # Parameters
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - impl Iterator<Item = &mut T> - An iterator over the row at the given y coordinate mutably.
            pub fn iter_row_mut(&mut self, y: usize) -> impl Iterator<Item = &mut T> {
                self.data[y * #width..(y + 1) * #width].iter_mut()
            }

            /// Returns the row at the given y coordinate or None if the coordinate is out of bounds.
            /// 
            /// # Parameters
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - Option<&[T]> - A reference to the row at the given y coordinate or None if the coordinate is out of bounds.
            pub fn get_row(&self, y: usize) -> Option<&[T]> {
                if y < #height {
                    Some(&self.data[y * #width..(y + 1) * #width])
                } else {
                    None
                }
            }

            /// Returns the row at the given y coordinate mutably or None if the coordinate is out of bounds.
            /// 
            /// # Parameters
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - Option<&mut [T]> - A mutable reference to the row at the given y coordinate or None if the coordinate is out of bounds.
            pub fn get_row_mut(&mut self, y: usize) -> Option<&mut [T]> {
                if y < #height {
                    Some(&mut self.data[y * #width..(y + 1) * #width])
                } else {
                    None
                }
            }

            /// Returns the row at the given y coordinate without checking bounds.
            /// 
            /// # Safety
            /// This method can panic or return wrong rows if the coordinate is out of bounds.
            /// 
            /// # Parameters
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - &[T] - A reference to the row at the given y coordinate.
            pub fn get_row_unchecked(&self, y: usize) -> &[T] {
                &self.data[y * #width..(y + 1) * #width]
            }
            
            /// Returns the row at the given y coordinate mutably without checking bounds.
            /// 
            /// # Safety
            /// This method can panic or return wrong rows if the coordinate is out of bounds.
            /// 
            /// # Parameters
            /// - y: usize - The y coordinate.
            /// 
            /// # Returns
            /// - &mut [T] - A mutable reference to the row at the given y coordinate.
            pub fn get_row_unchecked_mut(&mut self, y: usize) -> &mut [T] {
                &mut self.data[y * #width..(y + 1) * #width]
            }

            /// Iterates over the 2D array column by column.
            /// 
            /// # Returns
            /// - impl Iterator<Item = &[T]> - An iterator over the 2D array column by column.
            pub fn rows(&self) -> impl Iterator<Item = &[T]> {
                self.data.chunks_exact(#width)
            }
        }

        impl <T> std::ops::Index<usize> for #struct_name<T> {
            type Output = [T];
            fn index(&self, index: usize) -> &Self::Output {
                &self.data[index * #width..(index + 1) * #width]
            }
        }

        impl <T> std::ops::IndexMut<usize> for #struct_name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.data[index * #width..(index + 1) * #width]
            }
        }

        impl <T> std::ops::Index<(usize, usize)> for #struct_name<T> {
            type Output = T;
            fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
                self.get_unchecked(x, y)
            }
        }

        impl <T> std::ops::IndexMut<(usize, usize)> for #struct_name<T> {
            fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
                self.get_unchecked_mut(x, y)
            }
        }


        impl <T> std::default::Default for #struct_name<T>
        where
            T: Default + Copy,
        {
            fn default() -> Self {
                #struct_name::new(T::default())
            }
        }

        impl <T> std::fmt::Debug for #struct_name<T>
        where
            T: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let max_entry_width = self.data.iter().map(|entry| format!("{:?}", entry).len()).max().unwrap_or(0);
                for y in 0..#height {
                    for x in 0..#width {
                        write!(f, "{:>width$} ", format!("{:?}", self.get(x, y).unwrap()), width = max_entry_width)?;
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}