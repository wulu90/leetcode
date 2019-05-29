/**
 * 40. Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target),
 * find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * Each number in candidates may only be used once in the combination.
 *
 * Note:
 * All numbers (including target) will be positive integers.
 * The solution set must not contain duplicate combinations.
 *
 * Example 1:
 * Input: candidates = [10,1,2,7,6,1,5], target = 8,
 * A solution set is:
 * [
 *   [1, 7],
 *   [1, 2, 5],
 *   [2, 6],
 *   [1, 1, 6]
 * ]
 *
 * Example 2:
 * Input: candidates = [2,5,2,1,2], target = 5,
 * A solution set is:
 * [
 *   [1,2,2],
 *   [5]
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        if candidates.is_empty() {
            return result;
        }
        let mut nums = candidates;
        nums.sort();
        if target < nums[0] {
            return result;
        }
        Self::backtracking(&nums, &mut vec![], target, 0, &mut result);
        result
    }

    fn backtracking(
        nums: &Vec<i32>,
        v: &mut Vec<i32>,
        target: i32,
        start_index: usize,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(v.clone());
        } else {
            for i in start_index..nums.len() {
                if target >= nums[i]
                    && (i == start_index || i > start_index && nums[i] != nums[i - 1])
                {
                    v.push(nums[i]);
                    Self::backtracking(nums, v, target - nums[i], i + 1, result);
                    v.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn combination_sum2() {
        assert_eq!(
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
        );
        assert_eq!(
            vec![vec![1, 2, 2], vec![5]],
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)
        );
    }
}
