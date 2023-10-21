/*
 * @lc app=leetcode id=412 lang=javascript
 *
 * [412] Fizz Buzz
 */

// @lc code=start
/**
 * @param {number} n
 * @return {string[]}
 */
var fizzBuzz = function (n) {
  const answer = [];
  for (let i = 1; i <= n; i++) {
    let ans = "";
    if (i % 3 === 0) ans += "Fizz";
    if (i % 5 === 0) ans += "Buzz";
    if (!ans) ans += i;
    answer.push(ans);
  }
  return answer;
};
// @lc code=end
