/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;
        let goal = nums.len() - 1;

        // Solution 1:
        // let mut i = 0;
        // 'outer: while i < goal {
        //     steps += 1;
        //     let mut next = (0, 0);
        //     for j in 1..=nums[i] {
        //         let n = i + j as usize;
        //         if n >= goal {
        //             break 'outer;
        //         }
        //         let end = n + nums[n] as usize;
        //         if end > next.1 {
        //             next = (n, end)
        //         }
        //     }
        //     i = next.0;
        // }

        // Solution 2:
        let (mut current, mut farther) = (0, 0);
        for i in 0..goal {
            farther = farther.max(i + nums[i] as usize);
            if i == current {
                steps += 1;
                current = farther;
            }
        }

        steps
    }
}
// @lc code=end
