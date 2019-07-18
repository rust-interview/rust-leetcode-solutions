pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Solution::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution::single_number(vec![0,1,0,1,0,1,99]), 99);
    }
}
