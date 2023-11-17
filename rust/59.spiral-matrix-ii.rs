/*
 * @lc app=leetcode id=59 lang=rust
 *
 * [59] Spiral Matrix II
 */

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        let (mut top, mut right, mut bottom, mut left) = (0, n, n, 0);
        let mut k = 1;
        loop {
            if left == right {
                break;
            }
            for j in left..right {
                matrix[top][j] = k;
                k += 1;
            }
            top += 1;
            if top == bottom {
                break;
            }
            for i in top..bottom {
                matrix[i][right - 1] = k;
                k += 1;
            }
            right -= 1;
            if left == right {
                break;
            }
            for j in (left..right).rev() {
                matrix[bottom - 1][j] = k;
                k += 1;
            }
            bottom -= 1;
            if top == bottom {
                break;
            }
            for i in (top..bottom).rev() {
                matrix[i][left] = k;
                k += 1;
            }
            left += 1;
        }
        matrix
    }
}
// @lc code=end
