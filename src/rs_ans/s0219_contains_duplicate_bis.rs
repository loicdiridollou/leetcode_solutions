use std::collections::HashMap;

struct Solution {}
impl Solution {
    #![allow(dead_code)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(val) = map.get(&num) {
                if i as i32 - val <= k {
                    return true;
                }
            }
            map.insert(*num, i as i32);
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_219() {
        assert_eq!(
            Solution::contains_nearby_duplicate(Vec::<i32>::from([1, 2, 3, 1]), 3 as i32),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(Vec::<i32>::from([1, 0, 1, 1]), 1 as i32),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(Vec::<i32>::from([1, 2, 3, 1, 2, 3]), 1 as i32),
            false
        );
    }
}
