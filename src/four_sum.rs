/**
 * 18 4Sum
 *
 * Given an array nums of n integers and an integer target,
 * are there elements a, b, c, and d in nums such that a + b + c + d = target?
 * Find all unique quadruplets in the array which gives the sum of target.
 *
 * Note:
 * The solution set must not contain duplicate quadruplets.
 *
 * Example:
 * Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
 * A solution set is:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = Vec::new();
        if nums.len() < 4 {
            return v;
        }

        let mut nums = nums;
        let len = nums.len();
        nums.sort();

        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target {
                break;
            }
            if nums[i] + nums[len - 3] + nums[len - 2] + nums[len - 1] < target {
                continue;
            }
            for j in i + 1..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                if nums[i] + nums[j] + nums[j + 1] + nums[j + 2] > target {
                    break;
                }
                if nums[i] + nums[j] + nums[len - 2] + nums[len - 1] < target {
                    continue;
                }
                let (mut k, mut l) = (j + 1, len - 1);
                while k < l {
                    let sum = nums[i] + nums[j] + nums[k] + nums[l];
                    if sum == target {
                        v.push(vec![nums[i], nums[j], nums[k], nums[l]]);
                        while k < l && nums[k] == nums[k + 1] {
                            k += 1;
                        }
                        while k < l && nums[l] == nums[l - 1] {
                            l -= 1;
                        }
                        k += 1;
                        l -= 1;
                    } else if sum > target {
                        l -= 1;
                    } else {
                        k += 1;
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
    fn four_sum() {
        assert_eq!(
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)
        );
        assert_eq!(
            vec![vec![-5, 2, 2, 4], vec![-1, 0, 2, 2]],
            Solution::four_sum(vec![-1, 2, 2, -5, 0, -1, 4], 3)
        );
    }
}
