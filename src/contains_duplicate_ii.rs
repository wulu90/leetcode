/**
 * 219 Contains Duplicate Ⅱ
 *
 * Given an array of integers and an integer k,
 * find out whether there are two distinct indices i and j in the array such that nums[i] = nums[j]
 * and the absolute difference between i and j is at most k.
 *
 * Example 1:
 * Input: nums = [1,2,3,1], k = 3
 * Output: true
 *
 * Example 2:
 * Input: nums = [1,0,1,1], k = 1
 * Output: true
 *
 * Example 3:
 * Input: nums = [1,2,3,1,2,3], k = 2
 * Output: false
 *
 */
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                if i - map.get(&nums[i]).unwrap() <= k as usize {
                    return true;
                }
            }
            map.insert(nums[i], i);
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn contains_nearby_duplicate() {
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
        );
        assert_eq!(
            true,
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1)
        );
        assert_eq!(
            false,
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2)
        );
    }
}
