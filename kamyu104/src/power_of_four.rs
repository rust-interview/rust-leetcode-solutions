pub struct Solution {}
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_four() {
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(5), false);
    }
}

