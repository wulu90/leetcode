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
            String::from("aaaaaaaaaaaaaaaaaaaaaaaaa"),
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

}
