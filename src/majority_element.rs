use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let len = nums.len() as i32;
        let mut result = nums[0];
        for n in nums {
            if map.contains_key(&n) {
                map.insert(n, *(map.get(&n).unwrap()) + 1);
                if *(map.get(&n).unwrap()) > len / 2 {
                    result = n;
                }
            } else {
                map.insert(n, 1);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn majority_element() {
        assert_eq!(3, Solution::majority_element(vec![3]));
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
