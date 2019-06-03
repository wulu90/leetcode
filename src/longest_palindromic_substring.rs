/**
 * 5. Longest Palindromic Substring
 *
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
 *
 * Example 1:
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 * Example 2:
 * Input: "cbbd"
 * Output: "bb"
 *
 */

pub struct Solution;

impl Solution {
    pub fn longest_palidrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }
        let chars = s.chars().collect::<Vec<char>>();
        let (mut start, mut end, mut max) = (0, 0, 0);
        for i in 0..chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                let mut temp = 0;
                while temp <= i
                    && i + 1 + temp < chars.len()
                    && chars[i - temp] == chars[i + 1 + temp]
                {
                    if 2 * temp + 2 > max {
                        max = 2 * temp + 2;
                        start = i - temp;
                        end = i + 1 + temp;
                    }
                    temp += 1;
                }
            }
        }
        for i in 0..chars.len() - 1 {
            let mut temp = 1;
            while temp <= i && i + temp < chars.len() && chars[i - temp] == chars[i + temp] {
                if 2 * temp + 1 > max {
                    max = 2 * temp + 1;
                    start = i - temp;
                    end = i + temp;
                }
                temp += 1;
            }
        }
        s[start..=end].to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn longest_palidrome() {
        assert_eq!("bab", Solution::longest_palidrome("babad".to_owned()));
        assert_eq!("bb", Solution::longest_palidrome("bb".to_owned()));
        assert_eq!("aaaa", Solution::longest_palidrome("aaaa".to_owned()));
        assert_eq!("aaa", Solution::longest_palidrome("aaa".to_owned()));
        assert_eq!("", Solution::longest_palidrome("".to_owned()));
        assert_eq!("a", Solution::longest_palidrome("abcd".to_owned()));
    }
}