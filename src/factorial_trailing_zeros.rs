pub struct Solution;

impl Solution {
    pub fn trailing_zeros(n: i32) -> i32 {
        let mut f = n / 5;
        let mut s = 0;
        while f > 0 {
            s += f;
            f /= 5;
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn trailing_zeros() {
        assert_eq!(0, Solution::trailing_zeros(3));
        assert_eq!(49, Solution::trailing_zeros(200));
        assert_eq!(6, Solution::trailing_zeros(25));
        assert_eq!(536870902, Solution::trailing_zeros(2147483647));
    }
}
