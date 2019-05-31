/**
 * 50. Pow(x, n)
 *
 * Implement pow(x, n), which calculates x raised to the power n (x^n).
 *
 * Example 1:
 * Input: 2.00000, 10
 * Output: 1024.00000
 *
 * Example 2:
 * Input: 2.10000, 3
 * Output: 9.26100
 *
 * Example 3:
 * Input: 2.00000, -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 *
 * Note:
 * -100.0 < x < 100.0
 * n is a 32-bit signed integer, within the range [−2^31, 2^31 − 1]
 *
 */

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    // be careful: n may overflow;
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let (mut x, mut n) = (x, n as i64);
        if n < 0 {
            x = 1_f64 / x;
            n = -n;
        }
        let mut pow = 1_f64;
        while n > 0 {
            match n % 2 {
                0 => {
                    x *= x;
                    n /= 2;
                }
                _ => {
                    pow *= x;
                    n -= 1;
                }
            }
        }
        pow
    }

    pub fn my_pow_1(x: f64, n: i32) -> f64 {
        match n {
            0 => 1.0,
            1 => x,
            -1 => 1.0 / x,
            _ => match n % 2 {
                0 => Self::my_pow(x * x, n / 2),
                _ => Self::my_pow(x, n - 1) * x,
            },
        }
    }

    pub fn my_pow_2(x: f64, n: i32) -> f64 {
        match n.cmp(&0) {
            Ordering::Equal => 1_f64,
            Ordering::Less => match n.checked_abs() {
                Some(i) => 1_f64 / Self::my_pow(x, i),
                None => 1_f64 / (Self::my_pow(x, i32::max_value()) * x),
            },
            Ordering::Greater => match n % 2 {
                0 => Self::my_pow(x * x, n / 2),
                _ => Self::my_pow(x, n - 1) * x,
            },
        }
    }

}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn my_pow() {
        assert_eq!(1024.0_f64, Solution::my_pow(2.00000, 10));
        assert_eq!(9.261000000000001_f64, Solution::my_pow(2.10000, 3));
        assert_eq!(0.25000_f64, Solution::my_pow(2.00000, -2));
        assert_eq!(0.00002543114507074558_f64, Solution::my_pow(34.00515, -3));
        assert_eq!(1_f64, Solution::my_pow(1.00000, -2147483648));
    }
}