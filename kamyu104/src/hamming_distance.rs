// Time:  O(1)
// Space: O(1)

pub struct Solution1 {}
impl Solution1 {
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

pub struct Solution2 {}
impl Solution2 {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let z = format!("{:b}", x ^ y);
        z.chars().filter(|&c| c == '1').count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(Solution1::hamming_distance(1, 4), 2);
        assert_eq!(Solution2::hamming_distance(1, 4), 2);
    }
}
