pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut l, mut r) = (0, x / 2 + 1);
        while l <= r {
            let mid = (l + r) / 2;
            let midpow = mid as i64 * mid as i64;
            if midpow > x.into() {
                r = mid - 1;
            } else if midpow < x.into() {
                l = mid + 1;
            } else {
                return mid;
            }
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn my_sqrt() {
        assert_eq!(1, Solution::my_sqrt(1));
        assert_eq!(1, Solution::my_sqrt(2));
        assert_eq!(1, Solution::my_sqrt(3));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(5));
        assert_eq!(2, Solution::my_sqrt(6));
        assert_eq!(2, Solution::my_sqrt(7));
        assert_eq!(2, Solution::my_sqrt(8));
        assert_eq!(3, Solution::my_sqrt(9));
        assert_eq!(100, Solution::my_sqrt(10029));
        assert_eq!(8, Solution::my_sqrt(80));
        assert_eq!(46340, Solution::my_sqrt(2147483647));
    }
}
