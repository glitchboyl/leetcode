/*
 * @lc app=leetcode id=520 lang=javascript
 *
 * [520] Detect Capital
 */

// @lc code=start
/**
 * @param {string} word
 * @return {boolean}
 */
var detectCapitalUse = function (word) {
  if (word.length === 1) return true;

  // Solution 1:
  // const d = "A".charCodeAt();
  // const firstIsLowercase = word[0].charCodeAt() - d > 26;
  // const secondIsLowercase = word[1].charCodeAt() - d > 26;
  // if (firstIsLowercase && !secondIsLowercase) return false;
  // for (let i = 2; i < word.length; i++) {
  //   if (word[i].charCodeAt() - d > 26 !== secondIsLowercase) return false;
  // }
  // return true;

  // Solution 2:
  return (
    word === word.toUpperCase() ||
    word === word[0] + word.substring(1).toLowerCase()
  );
};
// @lc code=end
