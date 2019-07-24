// Time:  O(1)
// Space: O(1)
pub struct Solution1 {}
impl Solution1 {
    pub fn is_power_of_four(num: i32) -> bool {
        num > 0 && (num & (num - 1)) == 0 &&
               ((num & 0b01010101010101010101010101010101) == num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_four() {
        assert_eq!(Solution1::is_power_of_four(16), true);
        assert_eq!(Solution1::is_power_of_four(5), false);
    }
}

