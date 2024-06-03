/// Sorts the given mutable vector `arr` in ascending order using Selection Sort.
///
/// Selection sort works by iteratively finding the minimum element within the unsorted
/// sub-array and swapping it with the first element of that sub-array. This process
/// repeats until the entire array is sorted.
///
/// Time complexity: O(n^2)
/// Space complexity: O(1) (in-place sorting)
///
/// # Arguments
///
/// * `arr` - A mutable reference to the vector to be sorted. The elements must be

pub fn sort<T: Ord>(arr: &mut [T]) {
    let length = arr.len();
    let (mut current, mut minimum) = (0, 0);

    while current < length {
        for i in current + 1..length {
            if arr[i] < arr[minimum] {
                minimum = i;
            }
        }
        arr.swap(current, minimum);

        current += 1;
        minimum = current;
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