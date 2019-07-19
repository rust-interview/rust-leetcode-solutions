pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(_a: i32, _b: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(Solution::range_bitwise_and(1, 2), 3);
        assert_ne!(Solution::range_bitwise_and(1, 2), 4);
    }
}
