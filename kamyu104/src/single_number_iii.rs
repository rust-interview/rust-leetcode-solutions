// Time:  O(n)
// Space: O(1)

pub struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let x_xor_y: i32 = nums.iter().fold(0, |acc, &num| acc ^ num);
        let bit: i32 = x_xor_y & -x_xor_y;
        let mut result = vec![0; 2];
        nums.iter()
            .for_each(|i| result[((i & bit) != 0) as usize] ^= i);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![5, 3]);
    }
}
