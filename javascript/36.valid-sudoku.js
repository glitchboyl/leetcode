/*
 * @lc app=leetcode id=36 lang=javascript
 *
 * [36] Valid Sudoku
 */

// @lc code=start
/**
 * @param {character[][]} board
 * @return {boolean}
 */
var isValidSudoku = function (board) {
  const row = new Array(9).fill(0);
  const col = new Array(9).fill(0);
  const box = new Array(9).fill(0);

  for (let r = 0; r < 9; r++) {
    for (let c = 0; c < 9; c++) {
      const cell = board[r][c];
      if (cell === ".") continue;
      const b = ~~(r / 3) * 3 + ~~(c / 3);
      const d = 1 << (cell - 1);
      if ((row[r] & d) !== 0 || (col[c] & d) !== 0 || (box[b] & d) !== 0)
        return false;
      row[r] += d;
      col[c] += d;
      box[b] += d;
    }
  }
  return true;
};
// @lc code=end
