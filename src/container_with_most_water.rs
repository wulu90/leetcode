/**
 * 11 Container With Most Water
 *
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai).
 * n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0).
 * Find two lines, which together with x-axis forms a container, such that the container contains the most water.
 *
 * Note: You may not slant the container and n is at least 2.
 */

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, height.len() - 1);
        let mut maxarea = 0;
        while i < j {
            let temp = height[i].min(height[j]) * (j - i) as i32;
            maxarea = maxarea.max(temp);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        maxarea
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn max_area() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
