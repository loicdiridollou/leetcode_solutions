use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut vec: Vec<usize> = vec![1, 2];
        let mut index = 0;
        for i in 2..n {
            vec.push(vec[i as usize - 1] + vec[i as usize - 2]);
        }
        return vec[n as usize - 1] as i32;
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
