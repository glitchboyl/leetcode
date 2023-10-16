/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // Solution 1:
        // use std::collections::HashMap;
        // let mut memo = HashMap::from([(1, 1), (2, 2)]);
        // fn climb(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        //     match memo.get(&n) {
        //         Some(&r) => return r,
        //         None => {
        //             let r = climb(n - 1, memo) + climb(n - 2, memo);
        //             memo.insert(n, r);
        //             return r;
        //         }
        //     }
        // }
        // return climb(n, &mut memo);

        // Solution 2:
        // let mut dp = vec![1, 2];
        // for i in 2..(n as usize) {
        //     dp.push(dp[i - 2] + dp[i - 1]);
        // }
        // return dp[n as usize - 1];

        // Solution 3:
        let (mut p, mut q) = (0, 1);
        for i in 1..(n as usize) {
            let t = q;
            q = p + q;
            p = t;
        }
        return p + q;
    }
}
// @lc code=end
