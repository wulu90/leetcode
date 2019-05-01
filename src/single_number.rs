pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        for i in nums {
            x ^= i;
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn single_number() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }

}
