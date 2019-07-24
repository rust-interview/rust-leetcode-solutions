pub struct Solution {}
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_sum() {
        assert_eq!(Solution::get_sum(1, 2), 3);
        assert_eq!(Solution::get_sum(-2, 3), 1);
    }
}
