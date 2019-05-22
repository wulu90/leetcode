/**
 * 205 Isomorphic String
 * Given two strings s and t, determine if they are isomorphic.
 * Two strings are isomorphic if the characters in s can be replaced to get t.
 * All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character but a character may map to itself.
 *
 * Example 1:
 * Input: s = "egg", t = "add"
 * Output: true
 *
 * Example 2:
 * Input: s = "foo", t = "bar"
 * Output: false
 *
 * Example 3:
 * Input: s = "paper", t = "title"
 * Output: true
 *
 */
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut stmap: HashMap<char, char> = HashMap::new();
        let mut tsmap: HashMap<char, char> = HashMap::new();
        for (a, b) in s.chars().zip(t.chars()) {
            if stmap.contains_key(&a) {
                if stmap.get(&a) != Some(&b) {
                    return false;
                }
            } else {
                if tsmap.contains_key(&b) {
                    return false;
                }
                stmap.insert(a, b);
                tsmap.insert(b, a);
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn is_isomorphic() {
        assert_eq!(
            true,
            Solution::is_isomorphic(String::from("egg"), String::from("add"))
        );
        assert_eq!(
            false,
            Solution::is_isomorphic(String::from("ab"), String::from("aa"))
        );
        assert_eq!(
            false,
            Solution::is_isomorphic(String::from("foo"), String::from("bar"))
        );
        assert_eq!(
            true,
            Solution::is_isomorphic(String::from("paper"), String::from("title"))
        );
    }
}
