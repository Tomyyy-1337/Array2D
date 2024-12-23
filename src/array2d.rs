#[cfg(test)]
mod test {
    use utils_derive::create_array2d;

    #[test]
    fn test_create_array2d() {
        create_array2d!(3, 3);

        let mut array = Array2D_3_3::default(0);
        for i in 0..9 {
            array[i/3][i%3] = i as i32;
        }

        for i in 0..9 {
            assert_eq!(array[i/3][i%3], i as i32);
            assert_eq!(array.get(i%3, i/3), Some(&(i as i32)));
            assert_eq!(array.get_mut(i%3, i/3), Some(&mut(i as i32)));
        }
    }

    #[test]
    fn test_create_array2d_with_new() {
        create_array2d!(3, 3);

        let array = Array2D_3_3::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);

        for i in 0..9 {
            assert_eq!(array[i/3][i%3], i as i32);
            assert_eq!(array.get(i%3, i/3), Some(&(i as i32)));
        }
    }

    #[test]
    fn test_create_array2d_with_default() {
        create_array2d!(20, 10);

        let mut array = Array2D_20_10::default(0f64);

        for i in 0..9 {
            assert_eq!(array[i/3][i%3], 0.0);
            assert_eq!(array.get(i%3, i/3), Some(&0.0));
            assert_eq!(array.get_mut(i%3, i/3), Some(&mut 0.0));
            assert_eq!(array.get_unchecked(i%3, i/3), &0.0);
        }
    }

    #[test]
    fn test_iter() {
        create_array2d!(3, 3);

        let array = Array2D_3_3::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);

        for (i, value) in array.iter().enumerate() {
            assert_eq!(*value, i as i32);
        }
    }
}