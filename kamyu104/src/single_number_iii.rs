// Time:  O(n)
// Space: O(1)

pub struct Solution1 {}
impl Solution1 {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // Xor all the elements to get x ^ y.
        let x_xor_y: i32 = nums.iter().fold(0, |acc, &num| acc ^ num);

        // Get the last bit where 1 occurs by "x & ~(x - 1)"
        // Because -(x - 1) = ~(x - 1) + 1 <=> -x = ~(x - 1)
        // So we can also get the last bit where 1 occurs by "x & -x"
        let bit: i32 = x_xor_y & -x_xor_y;
        let mut result = vec![0; 2];
        nums.iter()
            .for_each(|i| result[((i & bit) != 0) as usize] ^= i);
        result
    }
}

pub struct Solution2 {}
impl Solution2 {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // Xor all the elements to get x ^ y.
        let mut x_xor_y: i32 = 0;
        for i in nums.iter() {
            x_xor_y ^= i;
        }

        // Get the last bit where 1 occurs.
        let bit: i32 = x_xor_y & -x_xor_y;

        // Get the subset of A where the number has the bit.
        // The subset only contains one of the two integers, call it x.
        // Xor all the elements in the subset to get x.
        let mut x: i32 = 0;
        nums.iter().for_each(|i| {
            if i & bit != 0 {
                x ^= i;
            }
        });
        vec![x, x_xor_y ^ x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution1::single_number(vec![1, 2, 1, 3, 2, 5]), vec![5, 3]);

        assert_eq!(Solution2::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    }
}
