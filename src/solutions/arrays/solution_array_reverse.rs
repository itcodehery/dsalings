// src/solutions/arrays/solution_array_reverse.rs
// Solution for exercises/arrays/array_reverse.rs

pub fn rotate_array(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    if n == 0 {
        return;
    }

    let k_effective = (k % n as i32) as usize;

    if k_effective == 0 {
        return; // No rotation needed
    }

    // Solution using multiple reversals:
    // 1. Reverse the entire array.
    // 2. Reverse the first k_effective elements.
    // 3. Reverse the remaining n - k_effective elements.

    // Reverse the entire array
    nums.reverse();

    // Reverse the first k_effective elements
    nums[0..k_effective].reverse();

    // Reverse the remaining n - k_effective elements
    nums[k_effective..n].reverse();
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
