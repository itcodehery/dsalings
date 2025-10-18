// Solution 3: Product of Array Except Self

pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut answer = vec![1; n];
    let mut prefix = 1;
    for i in 0..n {
        answer[i] = prefix;
        prefix *= nums[i];
    }
    let mut postfix = 1;
    for i in (0..n).rev() {
        answer[i] *= postfix;
        postfix *= nums[i];
    }
    answer
}
