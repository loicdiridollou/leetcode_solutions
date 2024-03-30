struct Solution {}

impl Solution {
    #![allow(dead_code)]
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        return rec1[0] < rec2[2] && rec2[0] < rec1[2] && rec1[1] < rec2[3] && rec2[1] < rec1[3];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_836() {
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
            true
        );
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
            false
        );
    }
}
