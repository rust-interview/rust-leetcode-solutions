// Time:  O(1)
// Space: O(1)

pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, mut n: i32) -> i32 {
        while m < n {  // Remove the last bit 1 until n <= m.
            n &= n - 1;
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    }
}
