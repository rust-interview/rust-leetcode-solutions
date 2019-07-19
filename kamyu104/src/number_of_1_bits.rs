pub struct Solution {}

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(Solution::hamming_weight(3), 3);
        assert_eq!(Solution::hamming_weight(4), 4);
    }
}
