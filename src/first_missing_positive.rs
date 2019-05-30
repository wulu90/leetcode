/**
 * 41. First Missing Positive
 *
 * Given an unsorted integer array, find the smallest missing positive integer.
 *
 * Example 1:
 * Input: [1,2,0]
 * Output: 3
 *
 * Example 2:
 * Input: [3,4,-1,1]
 * Output: 2
 *
 * Example 3:
 * Input: [7,8,9,11,12]
 * Output: 1
 *
 * Note:
 * Your algorithm should run in O(n) time and uses constant extra space.
 */

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                set.insert(nums[i]);
            }
        }
        for j in 0..set.len() {
            if !set.contains(&(j as i32 + 1)) {
                return j as i32 + 1;
            }
        }
        return if set.is_empty() {
            1
        } else {
            set.len() as i32 + 1
        };
    }

    pub fn first_missing_positive_1(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[(nums[i] - 1) as usize] != nums[i] {
                let val = nums[i];
                nums.swap(i, (val - 1) as usize);
            }
        }
        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        return (n + 1) as i32;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn first_missing_positive() {
        assert_eq!(1, Solution::first_missing_positive(vec![]));
        assert_eq!(1, Solution::first_missing_positive(vec![0]));
        assert_eq!(2, Solution::first_missing_positive(vec![1]));
        assert_eq!(1, Solution::first_missing_positive(vec![2]));
        assert_eq!(1, Solution::first_missing_positive(vec![-1]));
        assert_eq!(3, Solution::first_missing_positive(vec![0, 1, 2]));
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2]));
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(1, Solution::first_missing_positive(vec![7, 8, 9, 11, 12]));
    }

    #[test]
    fn first_missing_positive_1() {
        assert_eq!(1, Solution::first_missing_positive_1(vec![]));
        assert_eq!(1, Solution::first_missing_positive_1(vec![0]));
        assert_eq!(2, Solution::first_missing_positive_1(vec![1]));
        assert_eq!(1, Solution::first_missing_positive_1(vec![2]));
        assert_eq!(1, Solution::first_missing_positive_1(vec![-1]));
        assert_eq!(3, Solution::first_missing_positive_1(vec![0, 1, 2]));
        assert_eq!(3, Solution::first_missing_positive_1(vec![1, 2]));
        assert_eq!(2, Solution::first_missing_positive_1(vec![3, 4, -1, 1]));
        assert_eq!(5, Solution::first_missing_positive_1(vec![3, 4, -1, 1, 2]));
        assert_eq!(5, Solution::first_missing_positive_1(vec![3, 4, 1, 2]));
    }
}
