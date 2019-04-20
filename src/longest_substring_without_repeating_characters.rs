use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut strmap: HashMap<char, usize> = HashMap::new();
        let char_arr = s.char_indices();
        let (mut i, mut lls) = (0, 0);
        for (inx, c) in char_arr {
            if strmap.contains_key(&c) {
                i = std::cmp::max(strmap[&c] + 1, i);
            }
            lls = std::cmp::max(lls, inx - i + 1);
            strmap.insert(c, inx);
        }
        lls as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn length_of_longest_substring() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("abcabcbb"))
        );
        assert_eq!(
            1,
            Solution::length_of_longest_substring(String::from("bbbbbb"))
        );
        assert_eq!(
            3,
            Solution::length_of_longest_substring(String::from("pwwkew"))
        );
        assert_eq!(1, Solution::length_of_longest_substring(String::from(" ")));
        assert_eq!(1, Solution::length_of_longest_substring(String::from("a")));
    }
}
