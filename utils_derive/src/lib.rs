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

#[proc_macro]
pub fn create_array2d(input: TokenStream) -> TokenStream {
    let Array2DInput { width, height } = parse_macro_input!(input as Array2DInput);

    let len = width.base10_parse::<usize>().unwrap() * height.base10_parse::<usize>().unwrap();
    let width = width.base10_parse::<usize>().unwrap();
    let height = height.base10_parse::<usize>().unwrap();
    let struct_name = format_ident!("Array2D_{}_{}", width, height);

    let expanded = quote! {
        pub struct #struct_name<T> {
            pub width: usize,
            pub height: usize,
            pub data: [T; #len],
        }
    
        impl<T> #struct_name<T> {
            pub fn new(data: [T; #len]) -> Self {
                #struct_name {
                    width: #width,
                    height: #height,
                    data,
                }
            }

            pub fn default(value: T) -> Self
            where
                T: Copy,
            {
                #struct_name {
                    width: #width,
                    height: #height,
                    data: [value; #len],
                }
            }

            pub fn get(&self, x: usize, y: usize) -> Option<&T> {
                if x < self.width && y < self.height {
                    Some(&self.data[y * self.width + x])
                } else {
                    None
                }
            }

            pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
                if x < self.width && y < self.height {
                    Some(&mut self.data[y * self.width + x])
                } else {
                    None
                }
            }

            pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
                if x < self.width && y < self.height {
                    self.data[y * self.width + x] = value;
                    Some(())
                } else {
                    None
                }
            }

            fn get_unchecked(&self, x: usize, y: usize) -> &T {
                &self.data[y * self.width + x]
            }

            pub fn iter(&self) -> impl Iterator<Item = &T> {
                self.data.iter()
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
                self.data.iter_mut()
            }

            pub fn iter_row(&self, y: usize) -> impl Iterator<Item = &T> {
                self.data[y * self.width..(y + 1) * self.width].iter()
            }

            pub fn iter_row_mut(&mut self, y: usize) -> impl Iterator<Item = &mut T> {
                self.data[y * self.width..(y + 1) * self.width].iter_mut()
            }

            pub fn get_row(&self, y: usize) -> Option<&[T]> {
                if y < self.height {
                    Some(&self.data[y * self.width..(y + 1) * self.width])
                } else {
                    None
                }
            }

            pub fn get_row_mut(&mut self, y: usize) -> Option<&mut [T]> {
                if y < self.height {
                    Some(&mut self.data[y * self.width..(y + 1) * self.width])
                } else {
                    None
                }
            }
        }

        impl <T> std::ops::Index<usize> for #struct_name<T> {
            type Output = [T];
            fn index(&self, index: usize) -> &Self::Output {
                &self.data[index * self.width..(index + 1) * self.width]
            }
        }

        impl <T> std::ops::IndexMut<usize> for #struct_name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.data[index * self.width..(index + 1) * self.width]
            }
        }

        impl <T> std::fmt::Debug for #struct_name<T>
        where
            T: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for y in 0..self.height {
                    for x in 0..self.width {
                        write!(f, "{:?} ", self.get(x, y).unwrap())?;
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}