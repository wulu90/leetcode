pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut hmap = HashMap::new();
        hmap.insert('}', '{');
        hmap.insert(']', '[');
        hmap.insert(')', '(');
        let mut schar = s.chars();
        let mut vec: Vec<char> = Vec::new();
        while let Some(v) = schar.next() {
            if let Some(t) = hmap.get(&v) {
                if vec.pop() != Some(*t) {
                    return false;
                }
            } else {
                vec.push(v);
            }
        }

        vec.is_empty()
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
