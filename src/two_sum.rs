use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..(nums.len() - 1) {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, val) in nums.iter().enumerate() {
            if let Some(j) = map.get(&(target - val)) {
                return vec![i as i32, *j];
            }
            map.insert(*val, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn two_sum_test1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 7, 15], 14));
        assert_eq!(vec![] as Vec<i32>, Solution::two_sum(vec![2, 7, 11, 15], 3));
    }

    #[test]
    fn two_sum_hashmap() {
        let mut v = Solution::two_sum_hashmap(vec![2, 7, 11, 15], 9);
        v.sort();
        assert_eq!(vec![0, 1], v);
        let mut v = Solution::two_sum_hashmap(vec![2, 7, 7, 15], 14);
        v.sort();
        assert_eq!(vec![1, 2], v);
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::two_sum_hashmap(vec![2, 7, 11, 15], 3)
        );
    }
}
