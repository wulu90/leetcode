pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x;
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        } else {
            let mut right_half = 0;
            while x > right_half {
                right_half = right_half * 10 + x % 10;
                x = x / 10;
            }
            x == right_half || x == right_half / 10
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn is_palindrome() {
        assert_eq!(true, Solution::is_palindrome(131));
        assert_eq!(true, Solution::is_palindrome(0));
        assert_eq!(true, Solution::is_palindrome(1));
        assert_eq!(true, Solution::is_palindrome(11));
        assert_eq!(true, Solution::is_palindrome(131));
        assert_eq!(true, Solution::is_palindrome(1331));
        assert_eq!(true, Solution::is_palindrome(13531));
        assert_eq!(false, Solution::is_palindrome(13));
        assert_eq!(false, Solution::is_palindrome(135));
    }
}
