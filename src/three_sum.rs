/**
 * 15 3Sum
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0? Find all unique triplets in the array which gives the sum of zero.
 *
 * Note:
 * The solution set must not contain duplicate triplets.
 *
 * Example:
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 * A solution set is:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let mut v = Vec::new();
        for i in 0..nums.len() - 2 {
            //avoid duplicate
            if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
                let (mut j, mut k) = (i + 1, nums.len() - 1);
                while j < k {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        v.push(vec![nums[i], nums[j], nums[k]]);
                        //avoid duplicate
                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }
                        while j < k && nums[k] == nums[k - 1] {
                            k -= 1;
                        }
                        j += 1;
                        k -= 1;
                    } else if nums[i] + nums[j] + nums[k] < 0 {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn three_sum() {
        assert_eq!(vec![] as Vec<Vec<i32>>, Solution::three_sum(vec![]));
        assert_eq!(vec![] as Vec<Vec<i32>>, Solution::three_sum(vec![-1, 0]));
        assert_eq!(vec![vec![-1, 0, 1]], Solution::three_sum(vec![-1, 0, 1]));
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }
}
