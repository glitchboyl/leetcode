/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut result: Vec<i32> = Vec::with_capacity(2);
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(&j) => {
                    result.push(j);
                    result.push(i as i32);
                    break;
                }
                None => map.insert(num, i as i32),
            };
        }
        return result;
    }
}
// @lc code=end
