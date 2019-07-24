// Time:  O(1)
// Space: O(1)
pub struct Solution {}
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut ma = a.clone();
        let mut mb = b.clone();
        while mb != 0 {
            let carry = ma & mb;
            ma ^= mb;
            mb = carry << 1;
        }
        ma
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
