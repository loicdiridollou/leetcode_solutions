struct Solution {}

impl Solution {
    #![allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;

        while j < nums.len() {
            if nums[j] == val {
                j += 1
            } else {
                nums[i] = nums[j];
                i += 1;
                j += 1;
            }
        }
        return i as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut vec1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut vec1, 2), 5);
        assert_eq!(vec1[0..5], [0, 1, 3, 0, 4]);
        assert_eq!(Solution::remove_element(&mut vec![], 2), 0);
        assert_eq!(
            Solution::remove_element(&mut vec![1, 2, 2, 2, 2, 2, 2], 2),
            1
        );
        assert_eq!(
            Solution::remove_element(&mut vec![2, 2, 2, 2, 2, 2, 2], 2),
            0
        );
        assert_eq!(Solution::remove_element(&mut vec![1], 1), 0);
    }
}
