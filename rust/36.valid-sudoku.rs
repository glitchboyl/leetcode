/*
 * @lc app=leetcode id=36 lang=rust
 *
 * [36] Valid Sudoku
 */

// @lc code=start
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Solution 1: HashSet
        // use std::collections::HashSet;
        // let mut row_set = HashSet::new();
        // let mut col_set = HashSet::new();
        // let mut box_set = HashSet::new();
        // for row in 0..9 {
        //     for col in 0..9 {
        //         match board[row][col] {
        //             '.' => continue,
        //             cell => {
        //                 if !row_set.insert((row, cell)) {
        //                     return false;
        //                 }
        //                 if !col_set.insert((col, cell)) {
        //                     return false;
        //                 }
        //                 if !box_set.insert((row / 3 * 3 + col / 3, cell)) {
        //                     return false;
        //                 }
        //             }
        //         }
        //     }
        // }

        // Solution 2:
        let mut rows = [0; 9];
        let mut cols = [0; 9];
        let mut boxes = [0; 9];
        for r in 0..9 {
            for c in 0..9 {
                match board[r][c] {
                    '.' => continue,
                    cell => {
                        let b = (r / 3 * 3) + c / 3;
                        let d = 1 << (cell.to_digit(10).unwrap() - 1);
                        if rows[r] & d != 0 || cols[c] & d != 0 || boxes[b] & d != 0 {
                            return false;
                        }
                        rows[r] += d;
                        cols[c] += d;
                        boxes[b] += d;
                    }
                }
            }
        }
        true
    }
}
// @lc code=end
