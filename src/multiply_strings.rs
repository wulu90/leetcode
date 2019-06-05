/**
 * 43. Multiply Strings
 *
 * Given two non-negative integers num1 and num2 represented as strings,
 * return the product of num1 and num2, also represented as a string.
 *
 * Example 1:
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 *
 * Example 2:
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *
 * Note:
 * The length of both num1 and num2 is < 110.
 * Both num1 and num2 contain only digits 0-9.
 * Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 * You must not use any built-in BigInteger library or convert the inputs to integer directly.
 */

pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut v: Vec<u16> = vec![0; num1.len() + num2.len() + 1];
        for (i, c1) in num1.bytes().rev().enumerate() {
            for (j, c2) in num2.bytes().rev().enumerate() {
                let temp = ((c1 - 48) * (c2 - 48)) as u16 + v[i + j];
                v[i + j] = temp % 10;
                v[i + j + 1] += temp / 10;
            }
        }
        while v.len() > 1 && v.last() == Some(&0) {
            v.pop();
        }
        v.iter()
            .rev()
            .map(|x| char::from((*x + 48) as u8))
            .collect::<String>()
    }


    pub fn multiply_1(num1: String, num2: String) -> String {
        if num1 == String::from("0") || num2 == String::from("0") {
            return String::from("0");
        }
        let mut v: Vec<Vec<u8>> = vec![Vec::with_capacity(num1.len() + num2.len()); num2.len()];
        let mut inx = 0;
        for p in num2.bytes().rev() {
            v.push(Self::product(&num1, p, inx));
            inx += 1;
        }
        let mut res = String::with_capacity(num1.len() + num2.len());
        let mut carry = 0;
        for i in 0..v[v.len() - 1].len() {
            let mut sum: u16 = 0;
            for j in 0..v.len() {
                let mut temp = 0;
                if i < v[j].len() {
                    temp = v[j][i];
                }
                sum += temp as u16;
            }
            sum += carry;
            carry = sum / 10;
            res.push(char::from((sum % 10 + 48) as u8));
        }
        if carry >= 1 {
            res.push(char::from((carry + 48) as u8));
        }
        res.chars().rev().collect::<String>()
    }

    fn product(num: &String, p: u8, mut inx: usize) -> Vec<u8> {
        if p == 48 {
            return vec![0];
        }

        let mut res: Vec<u8> = Vec::with_capacity(num.len() + inx + 1);
        while inx > 0 {
            res.push(0);
            inx -= 1;
        }
        let mut carry = 0;
        for c in num.bytes().rev() {
            let temp = (c - 48) * (p - 48) + carry;
            carry = temp / 10;
            res.push(temp % 10);
        }
        if carry >= 1 {
            res.push(carry);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn multiply() {
        assert_eq!("6", Solution::multiply("2".to_owned(), "3".to_owned()));
        assert_eq!(
            "56088",
            Solution::multiply("123".to_owned(), "456".to_owned())
        );
        assert_eq!("0", Solution::multiply("0".to_owned(), "3".to_owned()));
        assert_eq!("0", Solution::multiply("2".to_owned(), "0".to_owned()));
        assert_eq!("2324725235680339589575434145174345450376468286967007130548581359508676923461769510883584014763890133705678997934", Solution::multiply("6964594125027226699998128707627435007621143975736747759751".to_owned(), "333791918659904899647584436187733004125181273682766434".to_owned()));
    }
}