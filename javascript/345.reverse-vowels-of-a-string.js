/*
 * @lc app=leetcode id=345 lang=javascript
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string}
 */
var reverseVowels = function (s) {
  // Solution 1:
  // const vowels = s.match(/[aeiouAEIOU]/g);
  // return s.replace(/[aeiouAEIOU]/g, (el) => vowels.pop());

  // Solution 2:
  const isVowel = (c) =>
    c === "a" ||
    c === "e" ||
    c === "i" ||
    c === "o" ||
    c === "u" ||
    c === "A" ||
    c === "E" ||
    c === "I" ||
    c === "O" ||
    c === "U";
  s = s.split("");
  let left = 0,
    right = s.length - 1;
  while (left < right) {
    while (left < right && !isVowel(s[left])) {
      left++;
    }
    while (left < right && !isVowel(s[right])) {
      right--;
    }
    if (left < right) {
      [s[left], s[right]] = [s[right], s[left]];
      left++;
      right--;
    }
  }
  return s.join("");
};
// @lc code=end
