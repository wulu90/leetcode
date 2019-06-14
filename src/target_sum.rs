/**
 * 494. Target Sum
 *
 * You are given a list of non-negative integers, a1, a2, ..., an, and a target, S.
 * Now you have 2 symbols + and -. For each integer, you should choose one from + and - as its new symbol.
 *
 * Find out how many ways to assign symbols to make sum of integers equal to target S.
 *
 * Example 1:
 * Input: nums is [1, 1, 1, 1, 1], S is 3.
 * Output: 5
 * Explanation:
 * -1+1+1+1+1 = 3
 * +1-1+1+1+1 = 3
 * +1+1-1+1+1 = 3
 * +1+1+1-1+1 = 3
 * +1+1+1+1-1 = 3
 * There are 5 ways to assign symbols to make the sum of nums be target 3.
 *
 * Note:
 * The length of the given array is positive and will not exceed 20.
 * The sum of elements in the given array will not exceed 1000.
 * Your output answer is guaranteed to be fitted in a 32-bit integer.
 *
 */

pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut ways = 0;
        Self::helper(0, 0, &nums, s, &mut ways);
        ways as i32
    }

    fn helper(inx: usize, sum: i32, nums: &Vec<i32>, s: i32, ways: &mut usize) {
        if inx == nums.len() && sum == s {
            *ways += 1;
        } else {
            if inx < nums.len() {
                Self::helper(inx + 1, sum + nums[inx], nums, s, ways);
                Self::helper(inx + 1, sum - nums[inx], nums, s, ways);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn find_target_sum_ways() {
        assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
    }
}