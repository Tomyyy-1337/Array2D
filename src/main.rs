use array2d::create_array2d;

create_array2d!(5,8);
create_array2d!(50,80);

fn main() {
    let mut array = Array2D_5_8::default(0u16);

    array.set_unchecked(1, 1, 1);
    array.set_unchecked(2, 2, 2);

    let array_big = Array2D_50_80::default(0u64);
    
    println!("{:?}", array);

}