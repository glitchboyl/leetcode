/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let mut flowerbed = flowerbed;
        let mut flowers = 0;
        let mut i = 0;
        while i < flowerbed.len() {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1;
                flowers += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        flowers >= n
    }
}
// @lc code=end
