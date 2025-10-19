// src/solutions/arrays/solution_array_contains.rs
// Solution for exercises/arrays/array_contains.rs
// Implements Floyd's Tortoise and Hare (Cycle Detection) algorithm.

pub fn find_duplicate(nums: &[i32]) -> i32 {
    // Phase 1: Find the intersection point of the two runners.
    let mut tortoise = nums[0] as usize;
    let mut hare = nums[0] as usize;

    loop {
        tortoise = nums[tortoise] as usize;
        hare = nums[nums[hare] as usize] as usize;
        if tortoise == hare {
            break;
        }
    }

    // Phase 2: Find the entrance to the cycle.
    let mut ptr1 = nums[0] as usize;
    let mut ptr2 = tortoise;

    while ptr1 != ptr2 {
        ptr1 = nums[ptr1] as usize;
        ptr2 = nums[ptr2] as usize;
    }

    ptr1 as i32
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

    #[test]
    fn test_find_duplicate_all_elements_are_distinct_except_one() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 8];
        assert_eq!(find_duplicate(&nums), 8);
    }
}
