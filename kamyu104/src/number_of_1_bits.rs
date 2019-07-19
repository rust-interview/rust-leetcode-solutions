// Time:  O(logn) = O(32)
// Space: O(1)

pub struct Solution {}
impl Solution {
    pub fn hamming_weight(mut n: u32) -> u8 {
        let mut cnt: u8 = 0;
        while n > 0 {
            cnt += 1;
            n &= n - 1;
        }
        return cnt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(
            Solution::hamming_weight(
                u32::from_str_radix("00000000000000000000000000001011", 2).unwrap()
            ),
            3
        );
        assert_eq!(
            Solution::hamming_weight(
                u32::from_str_radix("00000000000000000000000010000000", 2).unwrap()
            ),
            1
        );
        assert_eq!(
            Solution::hamming_weight(
                u32::from_str_radix("11111111111111111111111111111101", 2).unwrap()
            ),
            31
        );
    }
}
