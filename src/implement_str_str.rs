pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if haystack.is_empty() || haystack.len() < needle.len() {
            return -1;
        }

        // very very very slow
        for i in 0..haystack.len() {
            if &haystack[i..i + 1] == &needle[0..1] {
                if needle.len() == 1 {
                    return i as i32;
                }
                for j in 1..needle.len() {
                    if i + j < haystack.len() && &needle[j..j + 1] == &haystack[i + j..i + j + 1] {
                        if j == needle.len() - 1 {
                            return i as i32;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        -1
    }

    pub fn str_str_stdlib(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|n| n as i32).unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn str_str() {
        assert_eq!(
            -1,
            Solution::str_str(String::from("asfa"), String::from("sfda"))
        );
        assert_eq!(
            2,
            Solution::str_str(String::from("hello"), String::from("ll"))
        );
        assert_eq!(
            -1,
            Solution::str_str(String::from("aaaaaa"), String::from("bba"))
        );
        assert_eq!(
            0,
            Solution::str_str(String::from("aaaaaa"), String::from(""))
        );
        assert_eq!(
            -1,
            Solution::str_str(String::from("aaaaaa"), String::from("aaaaaaaaaaaaa"))
        );
        assert_eq!(
            -1,
            Solution::str_str(String::from("mississippi"), String::from("issipi"))
        );
        assert_eq!(0, Solution::str_str(String::from("a"), String::from("a")));
    }
}
