pub struct Solution;

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let alphas = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let mut s = String::new();
        let mut quotient = n;

        while quotient > 0 {
            let mut mods = quotient % 26;
            if mods == 0 {
                mods = 25;
            } else {
                mods -= 1;
            }
            s.insert(0, alphas[mods as usize]);

            quotient -= 1;
            quotient /= 26;
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn convert_to_title() {
        assert_eq!(String::from("A"), Solution::convert_to_title(1));
        assert_eq!(String::from("AB"), Solution::convert_to_title(28));
        assert_eq!(String::from("AZ"), Solution::convert_to_title(52));
        assert_eq!(String::from("ZY"), Solution::convert_to_title(701));
        assert_eq!(String::from("AAC"), Solution::convert_to_title(705));
        assert_eq!(String::from("Z"), Solution::convert_to_title(26));
        assert_eq!(String::from("XZ"), Solution::convert_to_title(650));
        assert_eq!(String::from("YA"), Solution::convert_to_title(651));
        assert_eq!(String::from(""), Solution::convert_to_title(0));
    }
}
