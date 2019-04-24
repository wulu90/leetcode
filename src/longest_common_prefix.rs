pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }

        for i in 0..strs[0].len() {
            let c = strs[0].chars().collect::<Vec<char>>()[i];

            for j in 1..strs.len() {
                let str_chars = strs[j].chars().collect::<Vec<char>>();
                if i == str_chars.len() || str_chars[i] != c {
                    return String::from(strs[0].get(0..i).unwrap());
                }
            }
        }
        strs[0].to_string()
    }

    pub fn longest_common_prefix_divide_conquer(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        } else {
            return String::from(Solution::common_prefix_mid(
                &strs,
                0,
                (strs.len() - 1) as i32,
            ));
        }
    }

    //pub fn common_prefix_mid(strs: &Vec<String>, i: i32, j: i32) -> &str {
    pub fn common_prefix_mid(strs: &Vec<String>, i: i32, j: i32) -> String {
        if i == j {
            return strs[i as usize].to_string();
        } else {
            let mid = (i + j) / 2;
            let left_common = Solution::common_prefix_mid(strs, 0, mid);
            let right_common = Solution::common_prefix_mid(strs, mid + 1, j);
            return Solution::get_common_prefix(left_common, right_common);
        }
    }

    //&str ---Time Limit Exceeded
    //string ---Time Limit Exceeded
    // ???? how
    pub fn get_common_prefix_str<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        for i in 0..str1.len() {
            if i == str2.len() || str1[i..i + 1] != str2[i..i + 1] {
                return &str1[0..i];
            }
        }
        return str1;
    }

    pub fn get_common_prefix(str1: String, str2: String) -> String {
        for i in 0..str1.len() {
            if i == str2.len() || str1[i..i + 1] != str2[i..i + 1] {
                return str1[0..i].to_string();
            }
        }
        return str1.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn longest_common_prefix() {
        assert_eq!(
            String::from("fl"),
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ])
        );

        assert_eq!(String::from(""), Solution::longest_common_prefix(vec![]));

        assert_eq!(
            String::from("flower"),
            Solution::longest_common_prefix(vec![String::from("flower")])
        );

        assert_eq!(
            String::from("fl"),
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("fli")
            ])
        );

        assert_eq!(
            String::from("flo"),
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flo")
            ])
        );

        assert_eq!(
            String::from("flow"),
            Solution::longest_common_prefix(vec![String::from("flow"), String::from("flow")])
        );
        assert_eq!(
            String::from("flo"),
            Solution::longest_common_prefix(vec![
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            ])
        );
    }

    #[test]
    fn longest_common_prefix_divide_conquer() {
        assert_eq!(
            String::from("flow"),
            Solution::longest_common_prefix_divide_conquer(vec![
                String::from("flow"),
                String::from("flow")
            ])
        );

        assert_eq!(
            String::from("fl"),
            Solution::longest_common_prefix_divide_conquer(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ])
        );

        assert_eq!(
            String::from(""),
            Solution::longest_common_prefix_divide_conquer(vec![])
        );

        assert_eq!(
            String::from("flower"),
            Solution::longest_common_prefix_divide_conquer(vec![String::from("flower")])
        );

        assert_eq!(
            String::from("fl"),
            Solution::longest_common_prefix_divide_conquer(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("fli")
            ])
        );

        assert_eq!(
            String::from("flo"),
            Solution::longest_common_prefix_divide_conquer(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flo")
            ])
        );

        assert_eq!(
            String::from("flo"),
            Solution::longest_common_prefix_divide_conquer(vec![
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            ])
        );
    }
}
