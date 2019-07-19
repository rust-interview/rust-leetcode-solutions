// Time:  O(1)
// Space: O(1)

pub struct Solution1 {}
pub struct Solution2 {}

impl Solution1 {
    pub fn range_bitwise_and(m: u32, mut n: u32) -> u32 {
        while m < n {  // Remove the last bit 1 until n <= m.
            n &= n - 1;
        }
        n
    }
}

impl Solution2 {
    pub fn range_bitwise_and(m: u32, mut n: u32) -> u32 {
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
        assert_eq!(Solution1::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution1::range_bitwise_and(0, 1), 0);

        assert_eq!(Solution2::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution2::range_bitwise_and(0, 1), 0);
    }
}
