use array2d::create_array2d;

create_array2d!(2,3);
create_array2d!(10,20);

fn main() {
    let mut array = Array2D_2_3::default(0);

    array.set(1, 2, 1);
    assert_eq!(array.get(1, 2), Some(&1));
}