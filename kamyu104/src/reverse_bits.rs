// Time : O(logn) = O(32)
// Space: O(1)

pub struct Solution {}

impl Solution {
    pub fn reverse_bits(mut n: u32) -> u32 {
        let mut result: u32 = 0;
        for _ in (0..32) {
            result <<= 1;
            result |= n & 1;
            n >>= 1;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(Solution::reverse_bits(u32::from_str_radix("00000010100101000001111010011100", 2).unwrap()), u32::from_str_radix("00111001011110000010100101000000", 2).unwrap());
        assert_eq!(Solution::reverse_bits(u32::from_str_radix("11111111111111111111111111111101", 2).unwrap()), u32::from_str_radix("10111111111111111111111111111111", 2).unwrap());
    }
}