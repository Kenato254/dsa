/// Sorts a mutable vector of integers in ascending order using the Insertion 
/// Sort algorithm.
///
/// This function implements Insertion Sort, which iterates through the vector, 
/// removing one unsorted element at a time and inserting it into its correct 
/// position within the sorted sub-list at the front of the vector.
///
/// # Arguments
///
/// * `arr` - A mutable reference to the vector of integers to be sorted.
pub fn sort(arr: &mut Vec<i32>) {
    let (length, mut sorted) = (arr.len(), 0);

    while sorted < length - 1 {
        let extract_unsorted_elem = arr.swap_remove(sorted + 1);
        let mut fetch_sorted_index = sorted + 1;

        for i in (0..sorted + 1).rev() {
            if extract_unsorted_elem < arr[i] {
                fetch_sorted_index -= 1;
            }
        }

        arr.insert(fetch_sorted_index, extract_unsorted_elem);
        sorted += 1;
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