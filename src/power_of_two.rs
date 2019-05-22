/**
 * 231 Power of two
 *
 * Given an integer, write a function to determine if it is a power of two.
 *
 * Example 1:
 *
 * Input: 1
 * Output: true
 * Explanation: 2^0 = 1
 * Example 2:
 *
 * Input: 16
 * Output: true
 * Explanation: 2^4 = 16
 * Example 3:
 *
 * Input: 218
 * Output: false
 */

pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        /*!
         * if n is power of two, the binary only has one 1.
         * 16 ‭  00010000‬  8 00001000 128 10000000
         * 16-1 00001111
         * 00010000 ‬& 00001111 = 00000000
         */
        n > 0 && n & n - 1 == 0
        // n > 0 && n.count_ones() == 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn is_power_of_two() {
        assert_eq!(false, Solution::is_power_of_two(-12));
        assert_eq!(false, Solution::is_power_of_two(-2147483648));
        assert_eq!(false, Solution::is_power_of_two(-16));
        assert_eq!(false, Solution::is_power_of_two(0));
        assert_eq!(false, Solution::is_power_of_two(11234));
        assert_eq!(true, Solution::is_power_of_two(16));
        assert_eq!(true, Solution::is_power_of_two(512));
        assert_eq!(true, Solution::is_power_of_two(1024));
    }
}
