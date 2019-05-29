/**
 * 39. Combination Sum
 *
 * Given a set of candidate numbers (candidates) (without duplicates) and a target number (target),
 * find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * The same repeated number may be chosen from candidates unlimited number of times.
 *
 * Note:
 * All numbers (including target) will be positive integers.
 * The solution set must not contain duplicate combinations.
 *
 * Example 1:
 * Input: candidates = [2,3,6,7], target = 7,
 * A solution set is:
 * [
 *   [7],
 *   [2,2,3]
 * ]
 *
 * Example 2:
 * Input: candidates = [2,3,5], target = 8,
 * A solution set is:
 * [
 *   [2,2,2,2],
 *   [2,3,3],
 *   [3,5]
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
            return;
        }
        for i in start_index..nums.len() {
            // pruning and skip duplicate
            if target >= nums[i] && (i == start_index || i > start_index && nums[i] != nums[i - 1])
            {
                v.push(nums[i]);
                Self::backtracking(nums, v, target - nums[i], i, result);
                v.pop();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn combination_sum() {
        assert_eq!(
            vec![vec![2, 2, 3], vec![7]],
            Solution::combination_sum(vec![2, 3, 6, 7], 7)
        );

        assert_eq!(
            vec![vec![2, 2, 3], vec![2, 5]],
            Solution::combination_sum(vec![2, 3, 5], 7)
        );
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            Solution::combination_sum(vec![2, 3, 5], 8)
        );

        //test duplicate;
        assert_eq!(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            Solution::combination_sum(vec![2, 3, 3, 5], 8)
        );
    }
}
