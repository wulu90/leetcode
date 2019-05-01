pub struct Solution;

impl Solution {
    /**
     * convert s to bytes array
     * save alphanumeric in vec
     * loop the s second time compare with the pop of  vec
     * in ascii
     * 0-9 : 48-57
     * A-Z : 65-90
     * a-z : 97-122
     */
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.into_bytes();
        let mut v = Vec::with_capacity(bytes.len());
        for by in &bytes {
            if (*by >= 48 && *by <= 57) || (*by >= 97 && *by <= 122) {
                v.push(*by);
            } else if *by >= 65 && *by <= 90 {
                v.push(*by + 32);
            }
        }
        for by in &bytes {
            if ((*by >= 48 && *by <= 57) || (*by >= 97 && *by <= 122)) && (*by != v.pop().unwrap())
            {
                return false;
            } else if (*by >= 65 && *by <= 90) && (*by != v.pop().unwrap() - 32) {
                return false;
            }
        }
        true
    }

    pub fn is_palindrome_stdlib(s: String) -> bool {
        let s = s.to_ascii_lowercase();
        let mut chars = s.chars().filter(char::is_ascii_alphanumeric);
        while let (Some(next), Some(next_back)) = (chars.next(), chars.next_back()) {
            if next != next_back {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn is_palindrome() {
        assert_eq!(
            true,
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"))
        );
        assert_eq!(false, Solution::is_palindrome(String::from("race a car")));
        assert_eq!(true, Solution::is_palindrome(String::from("   ")));
        assert_eq!(true, Solution::is_palindrome(String::from("")));
        assert_eq!(true, Solution::is_palindrome(String::from("a")));
        assert_eq!(true, Solution::is_palindrome(String::from("  a    ")));
        assert_eq!(true, Solution::is_palindrome(String::from("1a1")));
    }

    #[test]
    fn is_palindrome_stdlib() {
        assert_eq!(
            true,
            Solution::is_palindrome_stdlib(String::from("A man, a plan, a canal: Panama"))
        );
        assert_eq!(
            false,
            Solution::is_palindrome_stdlib(String::from("race a car"))
        );
        assert_eq!(true, Solution::is_palindrome_stdlib(String::from("   ")));
        assert_eq!(true, Solution::is_palindrome_stdlib(String::from("")));
        assert_eq!(true, Solution::is_palindrome_stdlib(String::from("a")));
        assert_eq!(
            true,
            Solution::is_palindrome_stdlib(String::from("  a    "))
        );
        assert_eq!(true, Solution::is_palindrome_stdlib(String::from("1a1")));
    }
}
