mod insertion_sort;
pub use insertion_sort::insertion_sort::*;

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
}
