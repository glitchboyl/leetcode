/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let mut i = 0;
        let goal = nums.len() - 1;
        'outer: while i < goal {
            steps += 1;
            let mut next = (0, 0);
            for j in 1..=nums[i] {
                let n = i + j as usize;
                if n >= goal {
                    break 'outer;
                }
                let end = n + nums[n] as usize;
                if end > next.1 {
                    next = (n, end)
                }
            }
            i = next.0;
        }
        steps
    }
}
// @lc code=end
