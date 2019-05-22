/**
 * 217 Contains Duplicate
 *
 * Given an array of integers, find if the array contains any duplicates.
 * Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.
 *
 * Example 1:
 * Input: [1,2,3,1]
 * Output: true
 *
 * Example 2:
 * Input: [1,2,3,4]
 * Output: false
 *
 * Example 3:
 * Input: [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *
 */
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for a in nums {
            if set.contains(&a) {
                return true;
            } else {
                set.insert(a);
            }
        }
        false
    }

    pub fn contains_duplicate_1(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        len != set.len()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn contains_duplicate() {
        assert_eq!(false, Solution::contains_duplicate(vec![1]));
        assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert_eq!(false, Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert_eq!(
            true,
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
        );
    }

    #[test]
    fn contains_duplicate_1() {
        assert_eq!(false, Solution::contains_duplicate_1(vec![1]));
        assert_eq!(true, Solution::contains_duplicate_1(vec![1, 2, 3, 1]));
        assert_eq!(false, Solution::contains_duplicate_1(vec![1, 2, 3, 4]));
        assert_eq!(
            true,
            Solution::contains_duplicate_1(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
        );
    }
}
