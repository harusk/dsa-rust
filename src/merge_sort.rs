pub mod merge_sort {
    fn merge<T>(arr: &mut Vec<T>, p: usize, q: usize, r: usize) 
    where
        T: Copy + PartialOrd + std::fmt::Debug
    {
        let arr_sub: Vec<T> = arr.to_vec();
        dbg!(&arr);
        let (left, right) = arr_sub.split_at_checked(q).unwrap();
        let n_l = q - p + 1;
        let n_r = r - q;
        let mut k = p;
        let mut i = 0; let mut j = 0;
        while i < n_l && j < n_r {
            if left[i] <= right[j] {
                arr[k] = left[i];
                i += 1;
            } else {
                arr[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < n_l {
            arr[k] = left[i];
            i += 1;
            k += 1;
        }

        while j < n_r {
            arr[k] = right[j];
            j += 1;
            k += 1;
        }
    }

    pub fn merge_sort<T>(mut arr: &mut Vec<T>, p: usize, r: usize)
    where 
        T: Copy + PartialOrd + std::fmt::Debug
    {
        if p >= r {
            return
        }
        let q = (p+r)/2;
        merge_sort(&mut arr, p, q);
        merge_sort(&mut arr, q, r);
        merge(&mut arr, p, q, r);
    }
}
