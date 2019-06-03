/**
 * 6. ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 * string convert(string s, int numRows);
 *
 * Example 1:
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 * Example 2:
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 */

pub struct Solution;

impl Solution {
    pub fn zigzag_convert_1(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let c: Vec<char> = s.chars().collect();
        let len = s.len() as i32;
        let k = len / (2 * num_rows - 2);
        let mo = len % (2 * num_rows - 2);
        let mut res = String::with_capacity(len as usize);
        for i in 0..num_rows {
            for j in 0..k {
                if i == 0 || i == num_rows - 1 {
                    let vertical = j * (2 * num_rows - 2) + i;
                    res.push(c[vertical as usize]);
                } else {
                    let vertical = j * (2 * num_rows - 2) + i;
                    res.push(c[vertical as usize]);
                    let diagnoal = (j + 1) * (2 * num_rows - 2) - i;
                    res.push(c[diagnoal as usize]);
                }
            }
            if mo > num_rows {
                let vertical = k * (2 * num_rows - 2) + i;
                res.push(c[vertical as usize]);
                let diagnoal = (k + 1) * (2 * num_rows - 2) - i;
                if diagnoal < len && i != 0 && i != num_rows - 1 {
                    res.push(c[diagnoal as usize]);
                }
            } else {
                let vertical = k * (2 * num_rows - 2) + i;
                if vertical < len {
                    res.push(c[vertical as usize]);
                }
            }
        }
        res
    }

    pub fn zigzag_convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let c: Vec<char> = s.chars().collect();
        let len = s.len();
        let num_rows: usize = num_rows as usize;
        let mut res = String::with_capacity(len as usize);
        let key = 2 * num_rows - 2;
        for i in 0..num_rows as usize {
            let mut j = 0;
            while j + i < len {
                res.push(c[j + i]);
                if i != 0 && i != num_rows - 1 && j + key - i < len {
                    res.push(c[j + key - i]);
                }
                j += key;
            }
        }
        res
    }

    pub fn zigzag_convert_2(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let len = s.len();
        let mut going_down = false;
        let mut cur_row = 0;
        let mut v: Vec<String> = vec![String::new(); num_rows as usize];
        let num_rows: usize = num_rows as usize;
        let mut res = String::with_capacity(len as usize);
        for c in s.chars() {
            v[cur_row].push(c);
            if cur_row == 0 || cur_row == num_rows - 1 {
                going_down = !going_down;
            }
            cur_row = if going_down { cur_row + 1 } else { cur_row - 1 }
        }
        for strs in v {
            res.push_str(&strs);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn zigzag_convert() {
        assert_eq!(
            "PAHNAPLSIIGYIR",
            Solution::zigzag_convert("PAYPALISHIRING".to_owned(), 3)
        );

        assert_eq!(
            "PINALSIGYAHRPI",
            Solution::zigzag_convert("PAYPALISHIRING".to_owned(), 4)
        );

        assert_eq!(
            "PAYPALISHIRING",
            Solution::zigzag_convert("PAYPALISHIRING".to_owned(), 14)
        );
        assert_eq!(
            "PAYPALISHIRIGN",
            Solution::zigzag_convert("PAYPALISHIRING".to_owned(), 13)
        );
        assert_eq!("ABCED", Solution::zigzag_convert("ABCDE".to_owned(), 4));
        assert_eq!(
            "PAYPALISHIRING",
            Solution::zigzag_convert("PAYPALISHIRING".to_owned(), 1)
        );
    }
}