// Time:  O(1)
// Space: O(1)
pub struct Solution {}
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut distance: i32 = 0;
        let mut z: i32 = x ^ y;
        while z != 0 {
            distance += 1;
            z &= z - 1;
        }
        distance
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
