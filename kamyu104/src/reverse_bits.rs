// Time : O(logn) = O(32)
// Space: O(1)

pub struct Solution {}

impl Solution {
    pub fn reverse_bits(a: i32, b: i32) -> i32 {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(Solution::reverse_bits(1, 2), 3);
        assert_ne!(Solution::reverse_bits(1, 2), 4);
    }
}
