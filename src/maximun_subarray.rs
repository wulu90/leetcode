pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max: i32 = i32::min_value();
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                max = std::cmp::max(max, sum);
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn max_sub_array() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
        assert_eq!(-2, Solution::max_sub_array(vec![-2]));
        assert_eq!(1, Solution::max_sub_array(vec![-2, 1]));
        assert_eq!(-1, Solution::max_sub_array(vec![-1, -2]));
    }
}
