// Make me pass the test!
//
// Exercise: Rotate Array
// Given an integer array `nums`, rotate the array to the right by `k` steps, where `k` is non-negative.
// The rotation should be performed in-place.
//
// For example:
// Input: nums = [1,2,3,4,5,6,7], k = 3
// Output: [5,6,7,1,2,3,4]
// Explanation:
// rotate 1 steps: [7,1,2,3,4,5,6]
// rotate 2 steps: [6,7,1,2,3,4,5]
// rotate 3 steps: [5,6,7,1,2,3,4]
//
// Input: nums = [-1,-100,3,99], k = 2
// Output: [3,99,-1,-100]
// Explanation:
// rotate 1 steps: [99,-1,-100,3]
// rotate 2 steps: [3,99,-1,-100]

pub fn rotate_array(nums: &mut Vec<i32>, k: i32) {
    // TODO: Implement the array rotation in-place.
    // Hints:
    // 1. Consider the effective number of rotations needed by using the modulo operator.
    // 2. A common approach involves multiple reversals:
    //    a. Reverse the entire array.
    //    b. Reverse the first `k` elements.
    //    c. Reverse the remaining `n - k` elements.
    // 3. Be careful with edge cases like an empty array, k = 0, or k being larger than the array length.

    // Your code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate_array(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        rotate_array(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn test_rotate_by_zero() {
        let mut nums = vec![1, 2, 3];
        rotate_array(&mut nums, 0);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_empty_array() {
        let mut nums = vec![];
        rotate_array(&mut nums, 5);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_rotate_single_element_array() {
        let mut nums = vec![1];
        rotate_array(&mut nums, 1);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_rotate_by_full_length() {
        let mut nums = vec![1, 2, 3, 4];
        rotate_array(&mut nums, 4);
        assert_eq!(nums, vec![1, 2, 3, 4]); // Array should be back to original state
    }

    #[test]
    fn test_rotate_k_greater_than_length() {
        let mut nums = vec![1, 2, 3, 4, 5];
        rotate_array(&mut nums, 7); // Equivalent to k=2 (7 % 5 = 2)
        assert_eq!(nums, vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_rotate_long_array() {
        let mut nums = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        rotate_array(&mut nums, 6);
        assert_eq!(nums, vec![50, 60, 70, 80, 90, 100, 10, 20, 30, 40]);
    }
}
