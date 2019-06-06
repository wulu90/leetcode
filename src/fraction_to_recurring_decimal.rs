/**
 * 166. Fraction to Recurring Decimal
 *
 * Given two integers representing the numerator and denominator of a fraction,
 * return the fraction in string format.
 *
 * If the fractional part is repeating, enclose the repeating part in parentheses.
 *
 * Example 1:
 * Input: numerator = 1, denominator = 2
 * Output: "0.5"
 *
 * Example 2:
 * Input: numerator = 2, denominator = 1
 * Output: "2"
 *
 * Example 3:
 * Input: numerator = 2, denominator = 3
 * Output: "0.(6)"
 *
 */

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_owned();
        }
        let mut res: String = String::new();
        if (numerator < 0 && denominator > 0) || (numerator > 0 && denominator < 0) {
            res.push('-');
        }
        let numerator: u64 = (numerator as i64).abs() as u64;
        let denominator: u64 = (denominator as i64).abs() as u64;
        let quotient = numerator / denominator;
        res += &quotient.to_string();
        let mut remainder = numerator % denominator;
        if remainder == 0 {
            return res;
        }

        let mut dec_res: String = String::new();
        let mut mod_set: HashMap<u64, usize> = HashMap::new();
        mod_set.insert(remainder, dec_res.len());
        while remainder != 0 {
            let temp = remainder * 10;
            dec_res += &((temp / denominator).to_string());
            remainder = temp % denominator;
            if mod_set.contains_key(&remainder) {
                dec_res.insert(*mod_set.get(&remainder).unwrap(), '(');
                dec_res.push(')');
                break;
            } else {
                mod_set.insert(remainder, dec_res.len());
            }
        }
        res + "." + &dec_res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn fraction_to_decimal() {
        assert_eq!("0", Solution::fraction_to_decimal(0, 2));
        assert_eq!("0.5", Solution::fraction_to_decimal(1, 2));
        assert_eq!("2", Solution::fraction_to_decimal(2, 1));
        assert_eq!("0.(6)", Solution::fraction_to_decimal(2, 3));
        assert_eq!("-0.(6)", Solution::fraction_to_decimal(-2, 3));
        assert_eq!("-0.(6)", Solution::fraction_to_decimal(2, -3));
        assert_eq!("0.(6)", Solution::fraction_to_decimal(-2, -3));
        assert_eq!("1.(6)", Solution::fraction_to_decimal(5, 3));
        assert_eq!("-1.(6)", Solution::fraction_to_decimal(5, -3));
        assert_eq!("-1.(6)", Solution::fraction_to_decimal(-5, 3));
        assert_eq!("1.(6)", Solution::fraction_to_decimal(-5, -3));
        assert_eq!("0.1(6)", Solution::fraction_to_decimal(1, 6));
        assert_eq!("0.041(6)", Solution::fraction_to_decimal(1, 24));
        assert_eq!("0.(0588235294117647)", Solution::fraction_to_decimal(1, 17));
        assert_eq!("2147483648", Solution::fraction_to_decimal(-2147483648, -1));
        assert_eq!(
            "0.0000000004656612873077392578125",
            Solution::fraction_to_decimal(-1, -2147483648)
        );
    }
}