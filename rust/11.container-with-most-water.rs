/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = height.iter().enumerate();
        let (mut left, mut right) = (iter.next(), iter.next_back());
        while let (Some((l, h1)), Some((r, h2))) = (left, right) {
            let w = (r - l) as i32;
            result = result.max(h1.min(h2) * w);
            if h1 < h2 {
                left = iter.next();
            } else {
                right = iter.next_back();
            }
        }
        return result;
    }
}
// @lc code=end
