/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits.clone();
        let mut r = result.pop().unwrap_or(0) + 1;
        if (r == 10) {
            r = 0;
            result = Self::plus_one(result);
        }
        result.push(r);
        return result;
    }
}
// @lc code=end
