use array2d::create_array2d;

create_array2d!(5,8);

fn main() {
    let mut array = Array2D_5_8::new(6u64);

    array.set_unchecked(1, 1, 1);
    array[2][2] = 2;

    println!("{:?}", array);
}