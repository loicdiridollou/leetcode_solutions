struct Solution {}

impl Solution {
    #![allow(dead_code)]
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut x_axis = Vec::<i32>::new();
        for point in &coordinates {
            x_axis.push(point[0]);
        }
        if x_axis
            .into_iter()
            .collect::<::std::collections::HashSet<_>>()
            .len()
            == 1
        {
            return true;
        }

        let mut coef = Vec::<f64>::new();
        for i in 1..coordinates.len() {
            if (coordinates[i - 1][0] == coordinates[i][0])
                & (coordinates[i - 1][1] != coordinates[i][1])
            {
                return false;
            }
            coef.push(
                (coordinates[i - 1][1] - coordinates[i][1]) as f64
                    / (coordinates[i - 1][0] - coordinates[i][0]) as f64,
            )
        }
        return coef
            .iter()
            .filter(|f| **f != coef[0])
            .collect::<Vec<_>>()
            .len()
            == 0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1232() {
        assert_eq!(
            Solution::check_straight_line(vec![
                [1, 2].to_vec(),
                [2, 3].to_vec(),
                [3, 4].to_vec(),
                [4, 5].to_vec(),
                [5, 6].to_vec(),
                [6, 7].to_vec()
            ]),
            true
        );
        assert_eq!(
            Solution::check_straight_line(vec![
                [1, 1].to_vec(),
                [2, 2].to_vec(),
                [3, 4].to_vec(),
                [4, 5].to_vec(),
                [5, 6].to_vec(),
                [7, 7].to_vec()
            ]),
            false
        );
    }
}
