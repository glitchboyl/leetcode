/*
 * @lc app=leetcode id=520 lang=rust
 *
 * [520] Detect Capital
 */

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() == 1 {
            return true;
        }
        let word = word.chars().collect::<Vec<_>>();
        let first_is_lowercase = word[0].is_lowercase();
        let second_is_lowercase = word[1].is_lowercase();
        if first_is_lowercase && !second_is_lowercase {
            return false;
        }
        for i in 2..word.len() {
            if word[i].is_lowercase() != second_is_lowercase {
                return false;
            }
        }
        true
    }
}
// @lc code=end
