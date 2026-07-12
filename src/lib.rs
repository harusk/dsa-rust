mod insertion_sort;
mod merge_sort;

pub use insertion_sort::insertion_sort::*;
pub use merge_sort::merge_sort::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_intengers_with_insertion_sort() {
        let mut arr = [9,5,6,7,2,4];
        let arr_result = [2,4,5,6,7,9];
        sort(&mut arr, 6);
        assert_eq!(arr, arr_result);
    }

    #[test]
    fn sort_float_with_insertion_sort() {
        let mut arr = [1.2, 3.45, 23.4, 10.2, 32.0];
        let arr_result = [1.2, 3.45, 10.2, 23.4, 32.0];
        sort(&mut arr, 5);
        assert_eq!(arr, arr_result);
    }

    #[test]
    fn sort_with_merge_sort() {
        let mut arr = vec![12, 3, 7, 9, 14, 6, 11, 2];
        let arr_result = vec![2, 3, 6, 7, 9, 11, 12, 14];
        merge_sort(&mut arr, 0, 8);
        assert_eq!(arr, arr_result);
    }
}
