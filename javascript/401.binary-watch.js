/*
 * @lc app=leetcode id=401 lang=javascript
 *
 * [401] Binary Watch
 */

// @lc code=start
/**
 * @param {number} turnedOn
 * @return {string[]}
 */
var readBinaryWatch = function(turnedOn) {
    function countOnes(num) {
      let ones = 0;
      while (num) {
        if (num & 1) ones++;
        num >>= 1;
      }
      return ones;
    }

    const times = [];
    if (turnedOn < 9) {
      for (let h = 0; h < 12; h++) {
        for (let m = 0; m < 60; m++) {
          if (countOnes(h) + countOnes(m) === turnedOn) times.push(`${h}:${m < 10 ? '0' : ''}${m}`)
        }
      }
    }
    return times;
};
// @lc code=end

