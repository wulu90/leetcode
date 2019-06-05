/**
 * 47. Permutations II
 *
 * Given a collection of numbers that might contain duplicates,
 * return all possible unique permutations.
 *
 * Example:
 * Input: [1,1,2]
 * Output:
 * [
 *   [1,1,2],
*    [1,2,1],
 *   [2,1,1]
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut v: Vec<Vec<i32>> = Vec::new();
        Self::backtracking(&nums, &mut vec![false; nums.len()], &mut vec![], &mut v);
        v
    }

    fn backtracking(
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        temp: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if temp.len() == nums.len() {
            res.push(temp.clone());
        } else {
            for i in 0..nums.len() {
                if used[i] || (i > 0 && nums[i - 1] == nums[i] && !used[i - 1]) {
                    continue;
                }
                temp.push(nums[i]);
                used[i] = true;
                Self::backtracking(nums, used, temp, res);
                temp.pop();
                used[i] = false;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn permute_unique() {
        assert_eq!(
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],],
            Solution::permute_unique(vec![1, 1, 2])
        );
        assert_eq!(
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],],
            Solution::permute_unique(vec![1, 2, 1])
        );
    }
}