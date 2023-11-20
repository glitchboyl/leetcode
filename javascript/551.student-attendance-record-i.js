/*
 * @lc app=leetcode id=551 lang=javascript
 *
 * [551] Student Attendance Record I
 */

// @lc code=start
/**
 * @param {string} s
 * @return {boolean}
 */
var checkRecord = function (s) {
  let absent = 0,
    late = 0;
  for (const record of s) {
    switch (record) {
      case "A":
        late = 0;
        absent++;
        if (absent === 2) return false;
        break;
      case "L":
        late++;
        if (late === 3) return false;
        break;
      default:
        late = 0;
    }
  }
  return true;
};
// @lc code=end
