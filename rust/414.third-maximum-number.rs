/*
 * @lc app=leetcode id=414 lang=rust
 *
 * [414] Third Maximum Number
 */

// @lc code=start
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let (mut first, mut second, mut third) = (i64::MIN, i64::MIN, i64::MIN);
        for n in nums {
            let n = n as i64;
            if n == third || n == second || n == first {
                continue;
            }
            if n > third {
                third = n;
                if n > second {
                    third = second;
                    second = n;
                    if n > first {
                        second = first;
                        first = n;
                    }
                }
            }
        }
        return (if third == i64::MIN { first } else { third }) as i32;
    }
}
// @lc code=end
