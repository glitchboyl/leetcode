/*
 * @lc app=leetcode id=96 lang=rust
 *
 * [96] Unique Binary Search Trees
 */

// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;

        // Solution 1:
        // let mut cache = vec![vec![0; n + 1]; n];
        // fn get_trees_num(start: usize, end: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
        //     if start > end {
        //         return 1;
        //     }
        //     if cache[start - 1][end] == 0 {
        //         for i in start..=end {
        //             cache[start - 1][end] +=
        //                 get_trees_num(start, i - 1, cache) * get_trees_num(i + 1, end, cache);
        //         }
        //     }
        //     cache[start - 1][end]
        // }
        // get_trees_num(1, n, &mut cache)

        // Solution 2: dp
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n]
    }
}
// @lc code=end
