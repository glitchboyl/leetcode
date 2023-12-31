/*
 * @lc app=leetcode id=6 lang=javascript
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
/**
 * @param {string} s
 * @param {number} numRows
 * @return {string}
 */
var convert = function (s, numRows) {
  if (numRows === 1) return s;

  const zigzag = [];
  for (let row = 0; row < numRows; row++) {
    let i = row;
    let rev = false;
    while (i < s.length) {
      zigzag.push(s[i]);
      let n = 0;
      if (row === 0 || row === numRows - 1) n = numRows - 1;
      else n = rev ? row : numRows - row - 1;
      i += 2 * n;
      rev = !rev;
    }
  }
  return zigzag.join("");
};
// @lc code=end
