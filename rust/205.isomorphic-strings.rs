/*
 * @lc app=leetcode id=205 lang=rust
 *
 * [205] Isomorphic Strings
 */

// @lc code=start
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s == t {
            return true;
        }

        // Solution 1: using HashMap
        use std::collections::HashMap;
        let mut s_map: HashMap<char, char> = HashMap::new();
        let mut t_map: HashMap<char, char> = HashMap::new();
        for (a, b) in s.chars().zip(t.chars()) {
            match (s_map.get(&a), t_map.get(&b)) {
                (Some(&c), Some(&d)) => {
                    if c != b || d != a {
                        return false;
                    }
                }
                (None, None) => {
                    s_map.insert(a, b);
                    t_map.insert(b, a);
                }
                _ => return false,
            }
        }

        // Solution 2: using Vec
        // let mut pairs: Vec<(char, char)> = Vec::new();
        // for (a, b) in s.chars().zip(t.chars()) {
        //     if let Some((c, d)) = pairs.iter().find(|(u, v)| (*u == a || *v == b)) {
        //         if a != *c || b != *d {
        //             return false;
        //         }
        //     } else {
        //         pairs.push((a, b));
        //     }
        // }

        return true;
    }
}
// @lc code=end
