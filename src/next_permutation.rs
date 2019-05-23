/**
 * 31 Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 * The replacement must be in-place and use only constant extra memory.
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 *
 * 1,2,3 → 1,3,2
 * 3,2,1 → 1,2,3
 * 1,1,5 → 1,5,1
 *
 */

pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let (mut k, mut l) = (-1, -1);
        for i in (0..=nums.len() - 2).rev() {
            if nums[i] < nums[i + 1] {
                k = i as i32;
                break;
            }
        }
        if k == -1 {
            nums.reverse();
            return;
        }
        for j in (k as usize + 1..=nums.len() - 1).rev() {
            if nums[k as usize] < nums[j] {
                l = j as i32;
                break;
            }
        }
        let temp = nums[k as usize];
        nums[k as usize] = nums[l as usize];
        nums[l as usize] = temp;
        nums[k as usize + 1..].reverse();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn next_permutation() {
        let mut v = vec![1, 2, 3];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1, 3, 2], v);
        v = vec![3, 2, 1];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1, 2, 3], v);
        v = vec![1, 1, 5];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1, 5, 1], v);
        v = vec![1, 2, 3, 4, 2];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1, 2, 4, 2, 3], v);
        v = vec![1];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1], v);
        v = vec![1, 2];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![2, 1], v);
        v = vec![2, 1];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1, 2], v);
    }
}
