use std::collections::HashSet;

// Time:  O(n)
// Space: O(n)

pub struct Solution {}
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for i in (0..32).rev() {
            result <<= 1;
            let mut prefixes: HashSet<i32> = HashSet::new();
            for n in nums.clone() {
                prefixes.insert(n >> i);
            }
            for p in prefixes.clone() {
                if prefixes.contains(&((result | 1) ^ p)) {
                    result += 1;
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_xor() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
    }
}
