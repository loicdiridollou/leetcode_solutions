struct Solution {}

impl Solution {
    #![allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::<String>::from(["".to_string()]);
        let mut map = std::collections::HashMap::new();
        if digits.len() == 0 {
            return Vec::<String>::new();
        }
        map.insert('2', vec!['a', 'b', 'c']);
        map.insert('3', vec!['d', 'e', 'f']);
        map.insert('4', vec!['g', 'h', 'i']);
        map.insert('5', vec!['j', 'k', 'l']);
        map.insert('6', vec!['m', 'n', 'o']);
        map.insert('7', vec!['p', 'q', 'r', 's']);
        map.insert('8', vec!['t', 'u', 'v']);
        map.insert('9', vec!['w', 'x', 'y', 'z']);

        for letter in digits.chars() {
            let t = res[0].len();
            while res[0].len() == t {
                let v = res.remove(0);
                for chr in map.get(&letter).unwrap() {
                    res.push(v.clone() + &chr.to_string())
                }
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_17() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations(String::from("")),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::letter_combinations(String::from("2")),
            vec!["a", "b", "c"]
        );
    }
}
