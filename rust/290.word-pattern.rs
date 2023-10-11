/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 */

// @lc code=start
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let mut p_map: HashMap<char, &str> = HashMap::new();
        let mut s_map: HashMap<String, char> = HashMap::new();
        let mut i = 0;
        let strs: Vec<&str> = s.split_whitespace().collect();
        if (strs.len() != pattern.chars().count()) {
            return false;
        }
        for p in pattern.chars() {
            match (p_map.get(&p), s_map.get(strs[i])) {
                (Some(s), Some(c)) => {
                    if s.to_owned() != strs[i] || c.to_owned() != p {
                        return false;
                    }
                }
                (None, None) => {
                    p_map.insert(p, strs[i]);
                    s_map.insert(strs[i].to_owned(), p);
                }
                _ => {
                    return false;
                }
            };
            i += 1;
        }
        return true;
    }
}
// @lc code=end
