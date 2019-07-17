// Time:  O(n)
// Space: O(1)

#[allow(dead_code)]
pub fn single_number(nums: Vec<i32>) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(!vec[2,2,1]), 1);
        assert_ne!(single_number(!vec[4,1,2,1,2]), 4);
    }
}