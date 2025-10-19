// Make me pass the test!
//
// Exercise: Find First Duplicate
// Given an array `nums` containing `n + 1` integers where each integer is between `1` and `n` (inclusive),
// prove that at least one duplicate number must exist. Assume that there is only one duplicate number,
// find the duplicate one.
//
// You must solve this problem without modifying the array `nums` and uses only constant extra space.
// You should also aim for a runtime complexity less than O(n^2).
//
// For example:
// Input: nums = [1,3,4,2,2]
// Output: 2
//
// Input: nums = [3,1,3,4,2]
// Output: 3

pub fn find_duplicate(nums: &[i32]) -> i32 {
    // TODO: Implement a solution to find the duplicate number.
    // Hints:
    // 1. Consider using the "Floyd's Tortoise and Hare" (cycle detection) algorithm.
    // 2. The array elements being between 1 and n allows them to be treated as pointers/indices.
    // 3. The problem constraints (no modifying array, constant extra space, < O(n^2) runtime)
    //    strongly suggest a cycle detection approach.

    0 // Placeholder, make this return the correct duplicate number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_example_1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(&nums), 2);
    }

    #[test]
    fn test_find_duplicate_example_2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(&nums), 3);
    }

    #[test]
    fn test_find_duplicate_short_array() {
        let nums = vec![1, 1];
        assert_eq!(find_duplicate(&nums), 1);
    }

    #[test]
    fn test_find_duplicate_larger_array() {
        let nums = vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1];
        assert_eq!(find_duplicate(&nums), 9);
    }

    #[test]
    fn test_find_duplicate_other_duplicate() {
        let nums = vec![4, 2, 1, 3, 4];
        assert_eq!(find_duplicate(&nums), 4);
    }

    #[test]
    fn test_find_duplicate_another_large_array() {
        let nums = vec![7, 9, 3, 1, 3, 4, 2, 5, 6, 8];
        assert_eq!(find_duplicate(&nums), 3);
    }
}
