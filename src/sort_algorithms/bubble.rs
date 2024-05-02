/// Sorts the given mutable vector `arr` in ascending order using Bubble Sort.
///
/// Bubble sort repeatedly iterates through the list, compares adjacent elements, and swaps
/// them if they are in the wrong order. Larger elements "bubble" up towards the end of
/// the list with each pass. The process stops when no swaps are needed in an entire pass,
/// indicating a sorted list.
///
/// Time complexity: O(n^2) (worst and average cases)
/// Space complexity: O(1) (in-place sorting)
///
/// # Arguments
///
/// * `arr` - A mutable reference to the vector to be sorted. The elements must be

pub fn sort(arr: &mut Vec<i32>) {
    let mut swap = true;
    let length = arr.len();

    while swap {
        swap = false;
        for i in 1..length {
            if arr[i - 1] > arr[i] {
                let temp = arr[i - 1];
                arr[i - 1] = arr[i];
                arr[i] = temp;
                swap = true;
            }
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