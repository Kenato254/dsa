use std::collections::HashMap;

/// Given am array of numbers with length >= 1 and a target return indexes of elements that add
/// upto the target
/// # Example
/// let nums =  vec![2,7,5,9];
/// let target = 9;
/// assert_eq!(two_sum(&nums. target), [0, 1]);

pub fn _two_sum(nums: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut result = (0,0);
    let mut seen:HashMap::<i32, i32> = HashMap::new();
    for (index, value) in nums.iter().enumerate() {
        let compliment = target - value;
        if seen.contains_key(&compliment) {
            result = (seen[&compliment], index as i32);
        }

        seen.insert(*value, index as i32);
    }

    result
}


#[cfg(test)]
mod tests {
    use super::_two_sum;

    #[test]
    fn two_sum_works() {
        let array = vec![2,7,5,9];
        let target = 9;

        let result = _two_sum(&array, target);
        let expected = (0, 1);

        assert_eq!(result, expected);
    }
}