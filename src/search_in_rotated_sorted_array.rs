/**
 * 33 Search in Rotated Sorted Array
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 * You are given a target value to search. If found in the array return its index, otherwise return -1.
 * You may assume no duplicate exists in the array.
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * Example 1:
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 *
 * Example 2:
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 *
 */

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        // where ever the mid is,
        // the left to mid is in ascending order, or
        // the mid to right is in ascending roder,
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            // determine whether left half or right half is ascending
            // if the target is in the ascending half, is like standard binary search
            // else is like recursive
            if nums[l] <= nums[mid] {
                if nums[l] <= target && target < nums[mid] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[r] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn search_in_rotated() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        assert_eq!(-1, Solution::search(vec![], 3));
        assert_eq!(-1, Solution::search(vec![1], 0));
        assert_eq!(0, Solution::search(vec![1], 1));
        assert_eq!(-1, Solution::search(vec![1, 3], 0));
        assert_eq!(-1, Solution::search(vec![1, 3], 2));
    }
}
