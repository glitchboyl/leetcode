/*
 * @lc app=leetcode id=500 lang=rust
 *
 * [500] Keyboard Row
 */

// @lc code=start
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let keyboard = [
            2, 3, 3, 2, 1, 2, 2, 2, 1, 2, 2, 2, 3, 3, 1, 1, 1, 1, 2, 1, 1, 3, 1, 3, 1, 3,
        ];
        words
            .into_iter()
            .filter(|word| {
                let lowercase_word = word.to_ascii_lowercase();
                let mut bytes = lowercase_word.bytes();
                let r = keyboard[(bytes.next().unwrap() - b'a') as usize];
                bytes.all(|c| keyboard[(c - b'a') as usize] == r)
            })
            .collect()
    }
}
// @lc code=end
