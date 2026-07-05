mod insertion_sort;
pub use insertion_sort::insertion_sort::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_with_insertion_sort() {
        let mut arr = [1,5,6,7,3,4];
        let arr_result = [1,3,4,5,6,7];
        sort(&mut arr, 6);
        assert_eq!(arr, arr_result);
    }
}
