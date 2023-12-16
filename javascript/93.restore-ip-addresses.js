/*
 * @lc app=leetcode id=93 lang=javascript
 *
 * [93] Restore IP Addresses
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string[]}
 */
var restoreIpAddresses = function (s) {
  const ipAddresses = [];
  if (s.length >= 4 && s.length <= 12) {
    function getRest(i = 0) {
      return s[i] === "0" ? 0 : s[i] <= 2 ? 2 : 1;
    }
    function backtrack(i, rest, ip) {
      const remainBlocks = 3 - ip.length;
      for (let d = 1; d <= rest + 1; d++) {
        const remain = s.length - i - d;
        if (remain >= remainBlocks && remain <= remainBlocks * 3) {
          const block = s.slice(i, i + d);
          if (
            block.length === 3 &&
            block[0] === "2" &&
            (block[1] > 5 || (block[1] === "5" && block[2] > 5))
          )
            break;
          ip.push(block);
          if (ip.length === 4) ipAddresses.push(ip.join("."));
          else backtrack(i + d, getRest(i + d), ip);
          ip.pop();
        }
        if (remain === 0) break;
      }
    }
    backtrack(0, getRest(), []);
  }
  return ipAddresses;
};
// @lc code=end
