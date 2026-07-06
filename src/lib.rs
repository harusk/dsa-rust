mod insertion_sort;
pub use insertion_sort::insertion_sort::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_with_insertion_sort() {
        let mut arr = [9,5,6,7,2,4];
        let arr_result = [2,4,5,6,7,9];
        sort(&mut arr, 6);
        assert_eq!(arr, arr_result);
    }
}
