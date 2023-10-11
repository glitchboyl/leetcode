/*
 * @lc app=leetcode id=303 lang=rust
 *
 * [303] Range Sum Query - Immutable
 */

// @lc code=start
struct NumArray {
    acc_sums: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut acc_sums = vec![0];
        for i in 0..nums.len() {
            acc_sums.push(acc_sums[i] + nums[i]);
        }
        return Self {
            acc_sums
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        return self.acc_sums[right as usize + 1] - self.acc_sums[left as usize];
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
// @lc code=end

