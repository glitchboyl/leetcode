/*
 * @lc app=leetcode id=79 lang=rust
 *
 * [79] Word Search
 */

// @lc code=start
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut used = vec![vec![false; board[0].len()]; board.len()];
        let mut is_exist = false;
        let word: Vec<char> = word.chars().collect();
        fn backtrack(
            board: &Vec<Vec<char>>,
            used: &mut Vec<Vec<bool>>,
            r: usize,
            c: usize,
            word: &Vec<char>,
            next: usize,
            is_exist: &mut bool,
        ) {
            if *is_exist {
                return;
            }
            used[r][c] = true;
            if next == word.len() {
                *is_exist = true;
                return;
            }
            if r < board.len() - 1 && !used[r + 1][c] && board[r + 1][c] == word[next] {
                backtrack(board, used, r + 1, c, word, next + 1, is_exist);
            }
            if c < board[0].len() - 1 && !used[r][c + 1] && board[r][c + 1] == word[next] {
                backtrack(board, used, r, c + 1, word, next + 1, is_exist);
            }
            if r > 0 && !used[r - 1][c] && board[r - 1][c] == word[next] {
                backtrack(board, used, r - 1, c, word, next + 1, is_exist);
            }
            if c > 0 && !used[r][c - 1] && board[r][c - 1] == word[next] {
                backtrack(board, used, r, c - 1, word, next + 1, is_exist);
            }
            used[r][c] = false;
        }
        'outer: for r in 0..board.len() {
            for c in 0..board[0].len() {
                if is_exist {
                    break 'outer;
                }
                if board[r][c] == word[0] {
                    backtrack(&board, &mut used, r, c, &word, 1, &mut is_exist);
                }
            }
        }
        is_exist
    }
}
// @lc code=end
