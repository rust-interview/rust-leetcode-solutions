// Time:  O(1)
// Space: O(1)
pub struct Solution1 {}
impl Solution1 {
    pub fn is_power_of_four(num: i32) -> bool {
        num > 0 && (num & (num - 1)) == 0 && ((num & 0b01010101010101010101010101010101) == num)
    }
}

// Time:  O(1)
// Space: O(1)
pub struct Solution2 {}
impl Solution2 {
    pub fn is_power_of_four(num: i32) -> bool {
        let mut n = num;
        while n != 0 && (n & 0b11) == 0 {
            n >>= 2;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_four() {
        assert_eq!(Solution1::is_power_of_four(16), true);
        assert_eq!(Solution1::is_power_of_four(5), false);

        assert_eq!(Solution2::is_power_of_four(16), true);
        assert_eq!(Solution2::is_power_of_four(5), false);
    }
}
