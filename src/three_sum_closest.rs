/**
 * 16 3Sum Closest
 *
 * Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
 *
 * Example:
 * Given array nums = [-1, 2, 1, -4], and target = 1.
 * The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 *
 */

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut result = nums[0] + nums[1] + nums[nums.len() - 1];
        for i in 0..nums.len() - 2 {
            if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
                let (mut j, mut k) = (i + 1, nums.len() - 1);
                while j < k {
                    let temp = nums[i] + nums[j] + nums[k];
                    if temp > target {
                        k -= 1;
                    } else {
                        j += 1;
                    }
                    result = if (result - target).abs() > (temp - target).abs() {
                        temp
                    } else {
                        result
                    };
                    println!("{},{},{}", i, j, k);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn three_sum_closest() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
        assert_eq!(0, Solution::three_sum_closest(vec![0, 2, 1, -3], 1));
    }
}
