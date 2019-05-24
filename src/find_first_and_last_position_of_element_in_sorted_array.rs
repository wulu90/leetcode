/**
 * 34. Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 * Your algorithm's runtime complexity must be in the order of O(log n).
 * If the target is not found in the array, return [-1, -1].
 *
 * Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 *
 * Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 */

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v: Vec<i32> = vec![-1, -1];
        if nums.is_empty() {
            return v;
        }
        let (mut l, mut r) = (0, nums.len() as i32 - 1);
        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid as usize] == target {
                v[0] = mid;
                v[1] = mid;
                let (mut i, mut j) = (mid - 1, mid + 1);
                while i >= l && nums[i as usize] == target {
                    v[0] = i;
                    i -= 1;
                }
                while j <= r && nums[j as usize] == target {
                    v[1] = j;
                    j += 1;
                }
                break;
            } else if nums[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn search_range() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
        assert_eq!(vec![3, 3], Solution::search_range(vec![5, 7, 7, 8, 10], 8));
        assert_eq!(vec![0, 4], Solution::search_range(vec![7, 7, 7, 7, 7], 7));
        assert_eq!(
            vec![1, 4],
            Solution::search_range(vec![5, 7, 7, 7, 7, 10], 7)
        );
        assert_eq!(
            vec![7, 10],
            Solution::search_range(vec![1, 2, 3, 3, 3, 3, 5, 7, 7, 7, 7, 10], 7)
        );
        assert_eq!(
            vec![1, 4],
            Solution::search_range(
                vec![5, 7, 7, 7, 7, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23],
                7
            )
        );
        assert_eq!(
            vec![0, 3],
            Solution::search_range(
                vec![7, 7, 7, 7, 10, 11, 12, 13, 16, 17, 18, 19, 20, 21, 22, 23],
                7
            )
        );
        assert_eq!(vec![0, 1], Solution::search_range(vec![7, 7], 7));
        assert_eq!(vec![0, 0], Solution::search_range(vec![7], 7));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![7], 6));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![7], 8));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));

    }
}