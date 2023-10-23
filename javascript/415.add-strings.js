/*
 * @lc app=leetcode id=415 lang=javascript
 *
 * [415] Add Strings
 */

// @lc code=start
/**
 * @param {string} num1
 * @param {string} num2
 * @return {string}
 */
var addStrings = function (num1, num2) {
  const d = "0".charCodeAt();
  num1 = num1.split("");
  num2 = num2.split("");
  let n1 = num1.pop(),
    n2 = num2.pop(),
    carry = 0;
  let result = "";
  while (n1 || n2 || carry) {
    let sum = carry;
    if (n1) {
      sum += n1.charCodeAt() - d;
      n1 = num1.pop();
    }
    if (n2) {
      sum += n2.charCodeAt() - d;
      n2 = num2.pop();
    }
    carry = ~~(sum / 10);
    result = (sum % 10) + result;
  }
  return result;
};
// @lc code=end
