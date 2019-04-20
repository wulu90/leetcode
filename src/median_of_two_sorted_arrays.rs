pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        let mut m = nums1.len();
        let mut n = nums2.len();
        if m > n {
            let temp = nums1.clone();
            nums1 = nums2.clone();
            nums2 = temp.clone();
            m = nums1.len();
            n = nums2.len();
        }

        let (mut m_left, mut m_right) = (0, m);
        let half = (m + n + 1) / 2;
        let max_l;
        let min_r;

        while m_left <= m_right {
            let m_p = (m_left + m_right) / 2;
            let n_p = half - m_p;
            if m_p < m_right && nums1[m_p] < nums2[n_p - 1] {
                m_left = m_p + 1;
            } else if m_p > m_left && nums1[m_p - 1] > nums2[n_p] {
                m_right = m_p - 1;
            } else {
                if m_p == 0 {
                    max_l = nums2[n_p - 1];
                } else if n_p == 0 {
                    max_l = nums1[m_p - 1];
                } else {
                    max_l = std::cmp::max(nums1[m_p - 1], nums2[n_p - 1]);
                }

                if (m + n) % 2 == 1 {
                    return max_l as f64;
                }

                if m_p == m {
                    min_r = nums2[n_p];
                } else if n_p == n {
                    min_r = nums1[m_p]
                } else {
                    min_r = std::cmp::min(nums1[m_p], nums2[n_p]);
                }

                return ((max_l + min_r) as f64 / 2.0) as f64;
            }
        }
        0.0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn find_median_sorted_arrays() {
        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
        );

        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![2, 3], vec![1])
        );
        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![2, 3], vec![5])
        );
        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
        );

        assert_eq!(
            2.5,
            Solution::find_median_sorted_arrays(vec![3, 4], vec![1, 2])
        );

        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![3, 4])
        );
        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5])
        );
        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![4, 5], vec![1, 2, 3])
        );

        assert_eq!(3.5, Solution::find_median_sorted_arrays(vec![], vec![3, 4]));
        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![1, 3, 6], vec![])
        );

        assert_eq!(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3])
        );

        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![3, 4], vec![2])
        );

        assert_eq!(
            3.0,
            Solution::find_median_sorted_arrays(vec![1], vec![3, 6])
        );

        assert_eq!(
            6.0,
            Solution::find_median_sorted_arrays(vec![9], vec![3, 6])
        );
    }
}
