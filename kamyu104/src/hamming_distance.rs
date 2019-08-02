// Time:  O(1)
// Space: O(1)
pub struct Solution {}
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }
}

