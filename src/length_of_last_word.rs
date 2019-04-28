pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut prelen = 0;
        let mut curlen = 0;
        for ch in s.chars() {
            if ch == ' ' {
                if curlen > 0 {
                    prelen = curlen;
                };
                curlen = 0
            } else {
                curlen += 1;
            }
        }
        if curlen != 0 {
            curlen
        } else {
            prelen
        }
    }

    pub fn length_of_last_word_fromback(s: String) -> i32 {
        let mut len = 0;
        let mut isfind = false;
        for ch in s.chars().rev() {
            if ch == ' ' && isfind {
                break;
            }
            if ch != ' ' {
                len += 1;
                isfind = true;
            }
        }
        len
    }

    pub fn length_of_last_word_stdlib(s: String) -> i32 {
        s.trim()
            .split_whitespace()
            .last()
            .map(|x| x.len())
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn length_of_last_word() {
        assert_eq!(
            7,
            Solution::length_of_last_word(String::from("aslkdjf sajkdfo"))
        );
        assert_eq!(7, Solution::length_of_last_word(String::from("aslkdjf")));
        assert_eq!(7, Solution::length_of_last_word(String::from(" sajkdfo")));
        assert_eq!(0, Solution::length_of_last_word(String::from(" ")));
        assert_eq!(1, Solution::length_of_last_word(String::from("a")));
        assert_eq!(1, Solution::length_of_last_word(String::from("a       ")));
        assert_eq!(1, Solution::length_of_last_word(String::from("    a   ")));
    }

    #[test]
    fn length_of_last_word_fromback() {
        assert_eq!(
            7,
            Solution::length_of_last_word_fromback(String::from("aslkdjf sajkdfo"))
        );
        assert_eq!(
            7,
            Solution::length_of_last_word_fromback(String::from("aslkdjf"))
        );
        assert_eq!(
            7,
            Solution::length_of_last_word_fromback(String::from(" sajkdfo"))
        );
        assert_eq!(0, Solution::length_of_last_word_fromback(String::from(" ")));
        assert_eq!(1, Solution::length_of_last_word_fromback(String::from("a")));
        assert_eq!(
            1,
            Solution::length_of_last_word_fromback(String::from("a       "))
        );
        assert_eq!(
            1,
            Solution::length_of_last_word_fromback(String::from("    a   "))
        );
    }

    #[test]
    fn length_of_last_word_stdlib() {
        assert_eq!(
            7,
            Solution::length_of_last_word_stdlib(String::from("aslkdjf sajkdfo"))
        );
        assert_eq!(
            7,
            Solution::length_of_last_word_stdlib(String::from("aslkdjf"))
        );
        assert_eq!(
            7,
            Solution::length_of_last_word_stdlib(String::from(" sajkdfo"))
        );
        assert_eq!(0, Solution::length_of_last_word_stdlib(String::from(" ")));
        assert_eq!(1, Solution::length_of_last_word_stdlib(String::from("a")));
        assert_eq!(
            1,
            Solution::length_of_last_word_stdlib(String::from("a       "))
        );
        assert_eq!(
            1,
            Solution::length_of_last_word_stdlib(String::from("    a   "))
        );
    }
}
