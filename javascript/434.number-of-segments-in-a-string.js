/*
 * @lc app=leetcode id=434 lang=javascript
 *
 * [434] Number of Segments in a String
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var countSegments = function(s) {
    let count = 0;
    let ns = false;
    for (const c of s) {
      if (c === ' ') count += +ns;
      ns = c !== ' ';
    }
    return count + ns;
};
// @lc code=end

