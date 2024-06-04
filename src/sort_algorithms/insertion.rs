/// Sorts a mutable vector of integers in ascending order using the Insertion 
/// Sort algorithm.
///
/// This function implements Insertion Sort, which iterates through the vector, 
/// removing one unsorted element at a time and inserting it into its correct 
/// position within the sorted sub-list at the front of the vector.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the a type that implements `Ord` trait
fn sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn test_sort_sorts() {
        let mut result = vec![9, 7, 1, 10, 0];
        sort(&mut result);
        assert_eq!(result, vec![0, 1, 7, 9, 10]);
    }
}