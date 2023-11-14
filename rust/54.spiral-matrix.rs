/*
 * @lc app=leetcode id=54 lang=rust
 *
 * [54] Spiral Matrix
 */

// @lc code=start
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut result = Vec::with_capacity(m * n);
        let (mut top, mut right, mut bottom, mut left) = (0, n, m, 0);
        loop {
            if left == right {
                break;
            }
            result.extend(&matrix[top][left..right]);
            top += 1;
            if top == bottom {
                break;
            }
            result.extend(matrix[top..bottom].iter().map(|row| row[right - 1]));
            right -= 1;
            if left == right {
                break;
            }
            result.extend(matrix[bottom - 1][left..right].iter().rev());
            bottom -= 1;
            if top == bottom {
                break;
            }
            result.extend(matrix[top..bottom].iter().rev().map(|row| row[left]));
            left += 1;
        }
        result
    }
}
// @lc code=end
