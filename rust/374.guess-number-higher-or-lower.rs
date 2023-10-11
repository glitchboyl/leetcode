/*
 * @lc app=leetcode id=374 lang=rust
 *
 * [374] Guess Number Higher or Lower
 */

// @lc code=start
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut min, mut max) = (1, n);
        while min < max {
            let mid = min + (max - min) / 2;
            match guess(mid) {
                -1 => max = mid - 1,
                1 => min = mid + 1,
                _ => return mid,
            }
        }
        return min;
    }
}
// @lc code=end
