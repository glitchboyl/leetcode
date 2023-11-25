/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        if target < matrix[0][0] || target > matrix[m - 1][n - 1] {
            return false;
        }
        match matrix.binary_search_by(|row| row[0].cmp(&target)) {
            Ok(_) => true,
            Err(r) => matrix[r - 1].binary_search(&target).is_ok(),
        }
    }
}
// @lc code=end
