struct Solution {}
impl Solution {
    #![allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let mut clean = String::new();
        for chr in s.chars() {
            if chr.is_alphabetic() {
                clean.push(chr.to_ascii_lowercase())
            } else if chr.is_numeric() {
                clean.push(chr)
            }
        }
        return clean == clean.chars().rev().collect::<String>();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
        assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
        assert_eq!(Solution::is_palindrome(String::from(" ")), true);
        assert_eq!(Solution::is_palindrome(String::from("0P")), false);
    }
}
