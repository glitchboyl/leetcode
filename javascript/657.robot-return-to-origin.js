/*
 * @lc app=leetcode id=657 lang=javascript
 *
 * [657] Robot Return to Origin
 */

// @lc code=start
/**
 * @param {string} moves
 * @return {boolean}
 */
var judgeCircle = function (moves) {
  let x = 0,
    y = 0;
  for (const d of moves) {
    if (d === "U") y--;
    else if (d === "R") x++;
    else if (d === "D") y++;
    else x--;
  }
  return x === 0 && y === 0;
};
// @lc code=end
