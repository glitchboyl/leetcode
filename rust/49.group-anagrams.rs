/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

// @lc code=start
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut anagrams = HashMap::new();

        for word in strs {
            let mut key: Vec<_> = word.chars().collect();
            key.sort_unstable();
            anagrams.entry(key).or_insert(vec![]).push(word);
        }

        anagrams.into_values().collect::<Vec<_>>()
    }
}
// @lc code=end
