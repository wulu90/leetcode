//use std::i32;

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rev = 0;
        while x != 0 {
            let temp = x % 10;
            x = x / 10;
            if rev > i32::max_value() / 10 || (rev == i32::max_value() / 10 && temp > 7) {
                return 0;
            }

            if rev < std::i32::MIN / 10 || (rev == std::i32::MIN / 10 && temp < -8) {
                return 0;
            }

            rev = rev * 10 + temp;
        }
        rev
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn reverse_int() {
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(21, Solution::reverse(120));
        assert_eq!(0, Solution::reverse(0));

        assert_eq!(0, Solution::reverse(2147483647));
        assert_eq!(0, Solution::reverse(-2147483648));

        assert_eq!(0, Solution::reverse(2147483646));
        assert_eq!(0, Solution::reverse(-2147483647));

        assert_eq!(2147447412, Solution::reverse(2147447412));
        assert_eq!(2147483641, Solution::reverse(1463847412));
        assert_eq!(0, Solution::reverse(1463847413));
    }
}
