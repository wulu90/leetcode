pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pascal_triangle: Vec<Vec<i32>> = Vec::new();
        for i in 0..num_rows as usize {
            let mut sub_arr: Vec<i32> = Vec::new();
            if i == 0 {
                sub_arr.push(1);
            } else {
                sub_arr.push(1);
                let mut j = 1;
                while j < i {
                    sub_arr.push(pascal_triangle[i - 1][j - 1] + pascal_triangle[i - 1][j]);
                    j += 1;
                }
                sub_arr.push(1);
            }

            pascal_triangle.push(sub_arr);
        }
        pascal_triangle
    }
}

#[cfg(test)]
mod tset {
    use super::Solution;

    #[test]
    fn pascal_triangle_generate() {
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            Solution::generate(5)
        );

        assert_eq!(vec![vec![1]], Solution::generate(1));

        assert_eq!(vec![vec![1], vec![1, 1]], Solution::generate(2));
    }
}
