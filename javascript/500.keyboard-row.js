/*
 * @lc app=leetcode id=500 lang=javascript
 *
 * [500] Keyboard Row
 */

// @lc code=start
/**
 * @param {string[]} words
 * @return {string[]}
 */
var findWords = function (words) {
  const keyboard = [
    2, 3, 3, 2, 1, 2, 2, 2, 1, 2, 2, 2, 3, 3, 1, 1, 1, 1, 2, 1, 1, 3, 1, 3, 1,
    3,
  ];
  const d = "a".charCodeAt();
  return words.filter((word) => {
    const lowercaseWord = word.toLowerCase();
    const r = keyboard[lowercaseWord[0].charCodeAt() - d];
    for (let i = 1; i < lowercaseWord.length; i++) {
      if (keyboard[lowercaseWord[i].charCodeAt() - d] !== r) return false;
    }
    return true;
  });
};
// @lc code=end
