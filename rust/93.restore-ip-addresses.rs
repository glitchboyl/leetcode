/*
 * @lc app=leetcode id=93 lang=rust
 *
 * [93] Restore IP Addresses
 */

// @lc code=start
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ip_addresses = Vec::new();
        if s.len() >= 4 && s.len() <= 12 {
            fn backtrack(
                s: &String,
                i: usize,
                mut rest: usize,
                ip: &mut Vec<String>,
                ip_addresses: &mut Vec<String>,
            ) {
                let remain_blocks = 3 - ip.len();
                for d in 1..=rest + 1 {
                    let remain = s.len() - i - d;
                    if remain >= remain_blocks && remain <= remain_blocks * 3 {
                        let block = s[i..i + d].to_string();
                        let block_bytes = block.as_bytes();
                        if block.len() == 3
                            && block_bytes[0] == b'2'
                            && (block_bytes[1] > b'5'
                                || (block_bytes[1] == b'5' && block_bytes[2] > b'5'))
                        {
                            break;
                        }
                        ip.push(block);
                        if ip.len() == 4 {
                            ip_addresses.push(ip.clone().join("."));
                        } else {
                            backtrack(
                                s,
                                i + d,
                                match s.as_bytes()[i + d] {
                                    b'0' => 0,
                                    b => {
                                        if b <= b'2' {
                                            2
                                        } else {
                                            1
                                        }
                                    }
                                },
                                ip,
                                ip_addresses,
                            );
                        }
                        ip.pop();
                    }
                    if remain == 0 {
                        break;
                    }
                }
            }
            backtrack(
                &s,
                0,
                match s.as_bytes()[0] {
                    b'0' => 0,
                    b => {
                        if b <= b'2' {
                            2
                        } else {
                            1
                        }
                    }
                },
                &mut vec![],
                &mut ip_addresses,
            );
        }
        ip_addresses
    }
}
// @lc code=end
