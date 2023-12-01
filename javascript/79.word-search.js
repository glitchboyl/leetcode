/*
 * @lc app=leetcode id=79 lang=javascript
 *
 * [79] Word Search
 */

// @lc code=start
/**
 * @param {character[][]} board
 * @param {string} word
 * @return {boolean}
 */
var exist = function (board, word) {
  let isExist = false;
  const m = board.length,
    n = board[0].length;
  word = word.split("");
  const used = new Array(m).fill(null).map(() => new Array(n).fill(false));
  function backtrack(r, c, next) {
    if (isExist) return;
    if (next === word.length) {
      isExist = true;
      return;
    }
    used[r][c] = true;
    if (r < m - 1 && !used[r + 1][c] && board[r + 1][c] === word[next])
      backtrack(r + 1, c, next + 1);
    if (c < n - 1 && !used[r][c + 1] && board[r][c + 1] === word[next])
      backtrack(r, c + 1, next + 1);
    if (r > 0 && !used[r - 1][c] && board[r - 1][c] === word[next])
      backtrack(r - 1, c, next + 1);
    if (c > 0 && !used[r][c - 1] && board[r][c - 1] === word[next])
      backtrack(r, c - 1, next + 1);
    used[r][c] = false;
  }
  for (let r = 0; r < m; r++) {
    for (let c = 0; c < n; c++) {
      if (isExist) return isExist;
      if (board[r][c] === word[0]) {
        backtrack(r, c, 1);
      }
    }
  }
  return isExist;
};
// @lc code=end
