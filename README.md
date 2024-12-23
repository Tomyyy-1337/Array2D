# Array 2D
Stores a Grid / 2D Array of fixed size using a single array.

## Features
* Create and modify 2D arrays
* Iterate over rows or the entire data
* Overrloades for Indexing using []

``` rust
use array2d::create_array2d;

create_array2d!(2,3);

fn main() {
    let mut array = Array2D_2_3::default(0);
    array.set(1, 2, 1);
    assert_eq!(array.get(1, 2), Some(&1));
}
```

# License
MIT
