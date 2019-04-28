pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = 0;
        let mut result = String::new();

        let mut achx = a.chars();
        let mut bchx = b.chars();
        for _ in 0..a.len().max(b.len()) {
            let x = achx.next_back().unwrap_or('0').to_digit(10).unwrap();
            let y = bchx.next_back().unwrap_or('0').to_digit(10).unwrap();
            let mut sum = x + y + carry;
            if sum >= 2 {
                carry = 1;
                sum %= 2;
            } else {
                carry = 0
            };

            result.insert(0, std::char::from_digit(sum, 10).unwrap());
        }
        if carry > 0 {
            result.insert(0, std::char::from_digit(1, 10).unwrap());
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn add_binary() {
        assert_eq!(
            "10".to_owned(),
            Solution::add_binary("1".to_owned(), "1".to_owned())
        );

        assert_eq!(
            "1".to_owned(),
            Solution::add_binary("0".to_owned(), "1".to_owned())
        );
        assert_eq!(
            "100".to_owned(),
            Solution::add_binary("11".to_owned(), "1".to_owned())
        );
        assert_eq!(
            "101".to_owned(),
            Solution::add_binary("11".to_owned(), "10".to_owned())
        );
        assert_eq!(
            "11001".to_owned(),
            Solution::add_binary("1110".to_owned(), "1011".to_owned())
        );
        assert_eq!(
            "1110".to_owned(),
            Solution::add_binary("1011".to_owned(), "11".to_owned())
        );
    }
}
