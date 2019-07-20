// Time:  O(1)
// Space: O(1)

pub struct Solution1 {}
impl Solution1 {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

pub struct Solution2 {}
impl Solution2 {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & !-n) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert_eq!(Solution1::is_power_of_two(1), true);
        assert_eq!(Solution1::is_power_of_two(16), true);
        assert_eq!(Solution1::is_power_of_two(218), false);

        assert_eq!(Solution2::is_power_of_two(1), true);
        assert_eq!(Solution2::is_power_of_two(16), true);
        assert_eq!(Solution2::is_power_of_two(218), false);
    }
}
