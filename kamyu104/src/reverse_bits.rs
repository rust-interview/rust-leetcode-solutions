// Time : O(logn) = O(32)
// Space: O(1)

pub struct Solution1 {}
pub struct Solution2 {}

impl Solution1 {
    pub fn reverse_bits(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(Solution1::reverse_bits(u32::from_str_radix("01110011001", 2).unwrap()), u32::from_str_radix("01110011001", 2).unwrap());
    }
}