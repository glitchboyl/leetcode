/*
 * @lc app=leetcode id=67 lang=javascript
 *
 * [67] Add Binary
 */

// @lc code=start
/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
var addBinary = function (a, b) {
  // Solution 1:
  // const a_digits = a.split(""),
  //   b_digits = b.split("");
  // let carry = 0;
  // const longerLength = Math.max(a_digits.length, b_digits.length);
  // let result = "";
  // for (let i = 0; i < longerLength; i++) {
  //   const ad = +a_digits.pop() || 0,
  //     bd = +b_digits.pop() || 0;
  //   let s = ad + bd + carry;
  //   if (s >= 2) {
  //     s -= 2;
  //     carry = 1;
  //   } else carry = 0;
  //   result = s + result;
  // }
  // if (carry) result = carry + result;
  // return result;

  // Solution 2:
  return (BigInt(`0b${a}`) + BigInt(`0b${b}`)).toString(2);
};
// @lc code=end
