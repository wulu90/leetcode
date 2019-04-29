pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut pre1 = 1;
        let mut pre2 = 2;
        for _ in 3..=n {
            let cur = pre1 + pre2;
            pre1 = pre2;
            pre2 = cur;
        }
        pre2
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn climb_stairs() {
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(89, Solution::climb_stairs(10));
        assert_eq!(1836311903, Solution::climb_stairs(45));
    }
}
