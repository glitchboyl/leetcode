/*
 * @lc app=leetcode id=17 lang=javascript
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function (digits) {
  const combinations = [];
  if (digits.length > 0) {
    const map = {
      2: "abc",
      3: "def",
      4: "ghi",
      5: "jkl",
      6: "mno",
      7: "pqrs",
      8: "tuv",
      9: "wxyz",
    };
    function backtrackInsert(n, combination) {
      if (n === digits.length) combinations.push(combination);
      else {
        for (const c of map[digits[n]]) {
          backtrackInsert(n + 1, combination + c);
        }
      }
    }
    backtrackInsert(0, "");
  }
  return combinations;
};
// @lc code=end
