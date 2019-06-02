/**
 * 22. Generate Parentheses
 *
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 * For example, given n = 3, a solution set is:
 * [
 *   "((()))",
 *   "(()())",
 *   "(())()",
 *   "()(())",
 *   "()()()"
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn generate_parentheses(n: i32) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        Self::helper(String::new(), 0, 0, n, &mut v);
        v
    }

    fn helper(s: String, left: i32, right: i32, n: i32, v: &mut Vec<String>) {
        if left == n && right == n {
            v.push(s.clone());
        }
        if left < n {
            Self::helper(s.clone() + "(", left + 1, right, n, v);
        }
        if right < n && right < left {
            Self::helper(s.clone() + ")", left, right + 1, n, v);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn generate_parentheses() {
        assert_eq!(
            "((())),(()()),(())(),()(()),()()()"
                .to_owned()
                .split(',')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>(),
            Solution::generate_parentheses(3)
        );
    }
}