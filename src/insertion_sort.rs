pub mod insertion_sort {
    enum ArrayError {
        IndexOutOfBounds,
    }

    fn check_bounds(arr: &[i32], n: usize) -> Result<&i32, ArrayError> {
        arr.get(n).ok_or(ArrayError::IndexOutOfBounds)
    }

    pub fn sort(arr: &mut [i32], n: usize) {
        match check_bounds(arr, n - 1) {
            Ok(_val) => (),
            Err(ArrayError::IndexOutOfBounds) => println!("IndexOutOfBounds")
        }

        for i in 1..n {
            let key = arr[i];
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j-1];
                j -= 1;
            }
            arr[j] = key;
        }
    }
}
