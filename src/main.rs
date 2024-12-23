use utils::create_array2d;

create_array2d!(3, 3);

fn main() {
    let mut array = Array2D_3_3::default(0);
    array[0][0] = 1;
    array[0][1] = 2;
    array[0][2] = 3;
    array[1][0] = 4;
    array[1][1] = 5;
    array[1][2] = 6;
    array[2][0] = 7;
    array[2][1] = 8;

    println!("{:?}", array);
}