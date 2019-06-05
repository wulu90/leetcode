/**
 * 46. Permutations
 *
 * Given a collection of distinct integers,
 * return all possible permutations.
 *
 * Example:
 * Input: [1,2,3]
 * Output:
 * [
 *   [1,2,3],
 *   [1,3,2],
 *   [2,1,3],
 *   [2,3,1],
 *   [3,1,2],
 *   [3,2,1]
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = Vec::new();
        Self::backtracking(&nums, &mut vec![], &mut v);
        v
    }

    fn backtracking(nums: &Vec<i32>, temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if temp.len() == nums.len() {
            res.push(temp.clone());
        } else {
            for i in 0..nums.len() {
                if !temp.contains(&nums[i]) {
                    temp.push(nums[i]);
                    Self::backtracking(nums, temp, res);
                    temp.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn permute() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            Solution::permute(vec![1, 2, 3])
        );
    }
}