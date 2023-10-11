/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut memo = HashMap::from([(1, 1), (2, 2)]);
        fn climb(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            match memo.get(&n) {
                Some(&r) => return r,
                None => {
                    let r = climb(n - 1, memo) + climb(n - 2, memo);
                    memo.insert(n, r);
                    return r;
                }
            }
        }

        return climb(n, &mut memo);
    }
}
// @lc code=end
