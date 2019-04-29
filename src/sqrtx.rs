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

    pub fn my_sqrt_newton(x: i32) -> i32 {
        let mut s: f64 = 1.0;
        let precision = 0.000001;
        while (s * s - x as f64).abs() > precision {
            s = (s + x as f64 / s) / 2.0;
        }
        println!("{}", s);
        s as i32
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

    #[test]
    fn my_sqrt_newton() {
        assert_eq!(1, Solution::my_sqrt_newton(1));
        assert_eq!(1, Solution::my_sqrt_newton(2));
        assert_eq!(1, Solution::my_sqrt_newton(3));
        assert_eq!(2, Solution::my_sqrt_newton(4));
        assert_eq!(2, Solution::my_sqrt_newton(5));
        assert_eq!(2, Solution::my_sqrt_newton(6));
        assert_eq!(2, Solution::my_sqrt_newton(7));
        assert_eq!(2, Solution::my_sqrt_newton(8));
        assert_eq!(3, Solution::my_sqrt_newton(9));
        assert_eq!(100, Solution::my_sqrt_newton(10029));
        assert_eq!(8, Solution::my_sqrt_newton(80));
        assert_eq!(46340, Solution::my_sqrt_newton(2147483647));
    }
}
