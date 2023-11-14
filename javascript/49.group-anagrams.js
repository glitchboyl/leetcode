/*
 * @lc app=leetcode id=49 lang=javascript
 *
 * [49] Group Anagrams
 */

// @lc code=start
/**
 * @param {string[]} strs
 * @return {string[][]}
 */
var groupAnagrams = function (strs) {
  const anagrams = {};
  for (const word of strs) {
    const anagram = word.split("").sort().join("");
    (anagrams[anagram] = anagrams[anagram] || []).push(word);
  }
  return Object.values(anagrams);
};
// @lc code=end
