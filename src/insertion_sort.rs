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

        let i: usize = 2;
        for i in i..n {
            let key = arr[i];
            let mut j = i - 1;
            while j > 0 {
                if arr[j] < key {
                    arr[j + 1] = key;
                    break;
                }
                arr[j + 1] = arr[j];
                j = j - 1;
            }
            arr[j + 1] = key;
        }
    }
}
