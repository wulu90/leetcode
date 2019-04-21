pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut cur = 0;
        let mut prev;
        let mut result = 0;
        let mut cs = s.chars();
        while let Some(c) = cs.next() {
            prev = cur;
            match c {
                'I' => cur = 1,
                'V' => cur = 5,
                'X' => cur = 10,
                'L' => cur = 50,
                'C' => cur = 100,
                'D' => cur = 500,
                'M' => cur = 1000,
                _ => cur = 0,
            }

            if prev < cur {
                result = result - prev + (cur - prev);
            } else {
                result += cur;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn roman_to_int() {
        assert_eq!(1, Solution::roman_to_int(String::from("I")));
        assert_eq!(3, Solution::roman_to_int(String::from("III")));
        assert_eq!(4, Solution::roman_to_int(String::from("IV")));
        assert_eq!(10, Solution::roman_to_int(String::from("X")));
        assert_eq!(9, Solution::roman_to_int(String::from("IX")));
        assert_eq!(15, Solution::roman_to_int(String::from("XV")));
        assert_eq!(18, Solution::roman_to_int(String::from("XVIII")));

        assert_eq!(19, Solution::roman_to_int(String::from("XIX")));
        assert_eq!(30, Solution::roman_to_int(String::from("XXX")));
        assert_eq!(80, Solution::roman_to_int(String::from("LXXX")));
        assert_eq!(99, Solution::roman_to_int(String::from("XCIX")));
        assert_eq!(102, Solution::roman_to_int(String::from("CII")));
        assert_eq!(300, Solution::roman_to_int(String::from("CCC")));
        assert_eq!(400, Solution::roman_to_int(String::from("CD")));
        assert_eq!(800, Solution::roman_to_int(String::from("DCCC")));
        assert_eq!(1437, Solution::roman_to_int(String::from("MCDXXXVII")));
        assert_eq!(3333, Solution::roman_to_int(String::from("MMMCCCXXXIII")));
        assert_eq!(3999, Solution::roman_to_int(String::from("MMMCMXCIX")));
    }
}
