/*
 * @lc app=leetcode id=455 lang=rust
 *
 * [455] Assign Cookies
 */

// @lc code=start
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut g = g;
        let mut s = s;
        g.sort_unstable();
        s.sort_unstable();
        let mut content = 0;
        let (mut i, mut j): (usize, usize) = (0, 0);
        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                content += 1;
                i += 1; 
            }
            j += 1;
        }
        content
    }
}
// @lc code=end
