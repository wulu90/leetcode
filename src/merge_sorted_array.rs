pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m + n - 1;
        let mut m = m - 1;
        let mut n = n - 1;
        while i >= 0 {
            if n >= 0 && (m == -1 || nums1[m as usize] < nums2[n as usize]) {
                nums1[i as usize] = nums2[n as usize];
                n -= 1;
            } else {
                nums1[i as usize] = nums1[m as usize];
                m -= 1;
            }

            i -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);

        let mut nums1 = vec![1, 2, 3];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 3, &mut nums2, 0);
        assert_eq!(vec![1, 2, 3], nums1);

        let mut nums1 = vec![1, 2, 3, 5, 7, 8, 9, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 7, &mut nums2, 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 5, 6, 7, 8, 9], nums1);

        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(vec![1], nums1);

        let mut nums1 = vec![];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 0, &mut nums2, 0);
        assert_eq!(vec![] as Vec<i32>, nums1);
    }
}
