pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(row_index as usize);
        if row_index == 0 {
            return vec![1];
        } else {
            let last_row = Solution::get_row(row_index - 1);
            result.push(1);
            let mut i: i32 = 1;
            while i < row_index {
                result.push(last_row[(i - 1) as usize] + last_row[i as usize]);
                i += 1;
            }
            result.push(1);
        }

        result
    }

    pub fn get_row_optimize(row_index: i32) -> Vec<i32> {
        let mut result = vec![0; (row_index + 1) as usize];
        result[0] = 1;
        for i in 0..row_index {
            for j in (1..i + 2).rev() {
                result[j as usize] += result[(j - 1) as usize];
                println!("i {} j {},{:?}", i, j, result);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn get_row_test() {
        assert_eq!(vec![1], Solution::get_row(0));
        assert_eq!(vec![1, 1], Solution::get_row(1));
        assert_eq!(vec![1, 2, 1], Solution::get_row(2));
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
        assert_eq!(vec![1, 4, 6, 4, 1], Solution::get_row(4));
        assert_eq!(
            vec![
                1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040,
                193536720, 354817320, 573166440, 818809200, 1037158320, 1166803110, 1166803110,
                1037158320, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100,
                13884156, 4272048, 1107568, 237336, 40920, 5456, 528, 33, 1
            ],
            Solution::get_row(33)
        );
    }

    #[test]
    fn get_row_optimize_test() {
        //assert_eq!(vec![1], Solution::get_row_optimize(0));
        //assert_eq!(vec![1, 1], Solution::get_row_optimize(1));
        //assert_eq!(vec![1, 2, 1], Solution::get_row_optimize(2));
        //assert_eq!(vec![1, 3, 3, 1], Solution::get_row_optimize(3));
        assert_eq!(vec![1, 4, 6, 4, 1], Solution::get_row_optimize(4));
        /*assert_eq!(
            vec![
                1, 33, 528, 5456, 40920, 237336, 1107568, 4272048, 13884156, 38567100, 92561040,
                193536720, 354817320, 573166440, 818809200, 1037158320, 1166803110, 1166803110,
                1037158320, 818809200, 573166440, 354817320, 193536720, 92561040, 38567100,
                13884156, 4272048, 1107568, 237336, 40920, 5456, 528, 33, 1
            ],
            Solution::get_row_optimize(33)
        );*/
    }
}
