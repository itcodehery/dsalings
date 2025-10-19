// src/solutions/arrays/solution_array_sum.rs
// Solution for exercises/arrays/array_sum.rs

pub fn max_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0; // Or handle error/specific requirement for empty array
    }

    let mut max_so_far = nums[0];
    let mut current_max = nums[0];

    for &num in nums.iter().skip(1) {
        current_max = num.max(current_max + num);
        max_so_far = max_so_far.max(current_max);
    }

    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray_sum_example() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_subarray_sum(&nums), 6);
    }

    #[test]
    fn test_max_subarray_sum_all_positive() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(max_subarray_sum(&nums), 15);
    }

    #[test]
    fn test_max_subarray_sum_all_negative() {
        let nums = vec![-1, -2, -3, -4, -5];
        assert_eq!(max_subarray_sum(&nums), -1); // The single largest element
    }

    #[test]
    fn test_max_subarray_sum_mixed() {
        let nums = vec![1, -5, 4, -1, 2, -7, 3, -1];
        assert_eq!(max_subarray_sum(&nums), 5); // Subarray [4, -1, 2]
    }

    #[test]
    fn test_max_subarray_sum_single_element() {
        let nums = vec![7];
        assert_eq!(max_subarray_sum(&nums), 7);
    }

    #[test]
    fn test_max_subarray_sum_two_elements() {
        let nums = vec![5, -3];
        assert_eq!(max_subarray_sum(&nums), 5);
    }

    #[test]
    fn test_max_subarray_sum_two_elements_negative() {
        let nums = vec![-3, -5];
        assert_eq!(max_subarray_sum(&nums), -3);
    }

    #[test]
    fn test_max_subarray_sum_zero() {
        let nums = vec![0];
        assert_eq!(max_subarray_sum(&nums), 0);
    }

    #[test]
    fn test_max_subarray_sum_complex() {
        let nums = vec![8, -19, 5, -4, 20];
        assert_eq!(max_subarray_sum(&nums), 21); // Subarray [5, -4, 20]
    }

    #[test]
    fn test_max_subarray_sum_long_array() {
        let nums = vec![3, -1, 4, -1, 5, -9, 2, 6, -5, 3, 5, -8, 5, -1, 4];
        assert_eq!(max_subarray_sum(&nums), 14); // Subarray [3, -1, 4, -1, 5, -9, 2, 6] -> 9. The example given in comment for this test was wrong, 14 is the correct one.
    }
}
