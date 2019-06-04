/**
 * 17. Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
 *
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 *
 * |----|-----|-----|
 * |  1 |  2  |  3  |
 * |    | abc | def |
 * |----|-----|-----|
 * |  4 |  5  |  6  |
 * | ghi| jkl | mno |
 * |----|-----|-----|
 * |  7 |  8  |  9  |
 * |pqrs| tuv | wxyz|
 * |----|-----|-----|
 * |  + |  0  |     |
 * |  * |     |  #  |
 * |----|-----|-----|
 *
 * Example:
 * Input: "23"
 * Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 */

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let keymap = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut deque: VecDeque<String> = VecDeque::new();
        deque.push_back(String::new());
        for letter in digits.bytes() {
            for _j in 0..deque.len() {
                let temp = deque.pop_front().unwrap();
                for c in keymap[(letter - 50) as usize].chars() {
                    let mut temp = temp.clone();
                    temp.push(c);
                    deque.push_back(temp);
                }
            }
        }
        Vec::from(deque)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn letter_combinations() {
        assert_eq!(
            "ad,ae,af,bd,be,bf,cd,ce,cf"
                .split(',')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>(),
            Solution::letter_combinations("23".to_owned())
        );

        assert_eq!(
            vec![] as Vec<String>,
            Solution::letter_combinations("".to_owned())
        );
    }
}