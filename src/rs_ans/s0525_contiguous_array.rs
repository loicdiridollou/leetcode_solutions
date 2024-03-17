use std::collections::HashMap;

struct Solution {}
impl Solution {
    #![allow(dead_code)]
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut mp = HashMap::new();
        let mut sum_val = 0;
        let mut max_len = 0;

        for i in 0..nums.len() {
            sum_val += {
                if nums[i] == 1 {
                    1
                } else {
                    -1
                }
            };
            if sum_val == 0 {
                max_len = i + 1
            } else if let Some(count) = mp.get(&sum_val) {
                max_len = max_len.max(i - count);
            } else {
                mp.insert(sum_val, i);
            }
        }
        return max_len as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_525() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }
}
