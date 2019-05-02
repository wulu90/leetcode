pub struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut n = 0;
        let mut len: u32 = 0;
        for ch in s.chars().rev() {
            /*
             * in leetcode
             * u8 4ms
             * i32 0ms
             */
            n += (26_i32).pow(len) * (ch as i32 - 64);
            //n += (26 as i32).pow(len) * ((ch as u8 - 'A' as u8 + 1) as i32);
            len += 1;
        }
        n as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn title_to_number() {
        assert_eq!(1, Solution::title_to_number(String::from("A")));
        assert_eq!(26, Solution::title_to_number(String::from("Z")));
        assert_eq!(27, Solution::title_to_number(String::from("AA")));
        assert_eq!(28, Solution::title_to_number(String::from("AB")));
        assert_eq!(701, Solution::title_to_number(String::from("ZY")));
    }
}
