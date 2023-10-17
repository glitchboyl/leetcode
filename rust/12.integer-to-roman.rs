/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut roman = String::new();
        let mut i = 1000;
        let map = |n: i32| -> Option<&str> {
            return match n {
                1 => Some("I"),
                2 => Some("II"),
                3 => Some("III"),
                4 => Some("IV"),
                5 => Some("V"),
                6 => Some("VI"),
                7 => Some("VII"),
                8 => Some("VIII"),
                9 => Some("IX"),
                10 => Some("X"),
                20 => Some("XX"),
                30 => Some("XXX"),
                40 => Some("XL"),
                50 => Some("L"),
                60 => Some("LX"),
                70 => Some("LXX"),
                80 => Some("LXXX"),
                90 => Some("XC"),
                100 => Some("C"),
                200 => Some("CC"),
                300 => Some("CCC"),
                400 => Some("CD"),
                500 => Some("D"),
                600 => Some("DC"),
                700 => Some("DCC"),
                800 => Some("DCCC"),
                900 => Some("CM"),
                1000 => Some("M"),
                2000 => Some("MM"),
                3000 => Some("MMM"),
                _ => None,
            };
        };
        while i > 0 {
            let d = num / i;
            if d > 0 {
                if let Some(s) = map(d * i) {
                    roman.push_str(s);
                    num -= d * i;
                }
            }
            i /= 10;
        }
        return roman;
    }
}
// @lc code=end
