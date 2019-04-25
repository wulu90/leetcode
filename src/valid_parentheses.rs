pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec: Vec<&str> = Vec::with_capacity(s.len());
        let mut hmap = HashMap::new();
        hmap.insert("(", ")");
        hmap.insert("[", "]");
        hmap.insert("{", "}");
        for i in 0..s.len() {
            let key = &s[i..i + 1];
            if hmap.contains_key(key) {
                vec.push(key);
            } else {
                match vec.pop() {
                    Some(v) => match hmap.get(v) {
                        Some(t) => {
                            if t == &key {
                                continue;
                            } else {
                                return false;
                            }
                        }
                        None => {
                            return false;
                        }
                    },
                    None => {
                        return false;
                    }
                }
            }
        }
        vec.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn is_valid_parentheses() {
        assert_eq!(true, Solution::is_valid(String::from("()")));
        assert_eq!(true, Solution::is_valid(String::from("()[]{}")));
        assert_eq!(false, Solution::is_valid(String::from("(]")));
        assert_eq!(false, Solution::is_valid(String::from("([)]")));
        assert_eq!(true, Solution::is_valid(String::from("{[]}")));
        assert_eq!(false, Solution::is_valid(String::from("{")));
        assert_eq!(false, Solution::is_valid(String::from("}")));
    }
}
