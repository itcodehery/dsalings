// Solution 1: Rotate Array

pub fn rotate(nums: &mut [i32], k: i32) {
    let k = k as usize % nums.len();
    nums.rotate_right(k);
}