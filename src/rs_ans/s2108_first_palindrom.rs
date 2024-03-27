struct Solution {}

impl Solution {
    #![allow(dead_code)]
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words.iter() {
            if is_palindromic(word) {
                return word.to_string();
            }
        }
        return String::from("");
    }
}

fn is_palindromic(word: &str) -> bool {
    let mut lo = 0 as usize;
    let mut hi = word.len() - 1;
    let bytes = word.as_bytes();
    while lo < hi && bytes[lo] == bytes[hi] {
        lo += 1;
        hi -= 1;
    }

    hi <= lo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2108() {
        assert_eq!(
            Solution::first_palindrome(vec![
                "abc".to_string(),
                "car".into(),
                "ada".into(),
                "racecar".into(),
                "cool".into()
            ]),
            String::from("ada")
        );
        assert_eq!(
            Solution::first_palindrome(vec!["notapalindrome".to_string(), "racecar".to_string()]),
            String::from("racecar")
        );
        assert_eq!(
            Solution::first_palindrome(vec!["notapalindrome".to_string(), "eracecar".to_string()]),
            String::from("")
        );
    }
}
