pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut prelen = 0;
        let mut curlen = 0;
        let mut char_inx = s.char_indices();
        while let Some((_, y)) = char_inx.next() {
            if y == ' ' {
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

        assert_eq!(
            5,
            Solution::length_of_last_word(String::from("Hello World"))
        );

        assert_eq!(0, Solution::length_of_last_word(String::from(" ")));

        assert_eq!(1, Solution::length_of_last_word(String::from("a")));
        assert_eq!(1, Solution::length_of_last_word(String::from("a       ")));
        assert_eq!(1, Solution::length_of_last_word(String::from("    a   ")));
    }
}
