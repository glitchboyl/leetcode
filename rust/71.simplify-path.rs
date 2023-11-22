/*
 * @lc app=leetcode id=71 lang=rust
 *
 * [71] Simplify Path
 */

// @lc code=start
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut paths = Vec::new();
        for p in path.split('/') {
            match p {
                "" | "." => continue,
                ".." => {
                    paths.pop();
                }
                _ => {
                    paths.push(p);
                }
            };
        }
        format!("/{}", paths.join("/"))
    }
}
// @lc code=end
