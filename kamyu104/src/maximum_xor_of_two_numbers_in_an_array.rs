pub struct Solution {}
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_xor() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
    }
}
