// Time:  O(n)
// Space: O(1)
pub struct Solution1 {}
impl Solution1 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut num: i32 = 0;
        for i in 0..nums.len() {
            num ^= nums[i] ^ (i + 1) as i32;
        }
        num
    }
}

// Time:  O(n)
// Space: O(n)
// pub struct Solution2 {}
// impl Solution2 {
//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         0
//     }
// }

// Time:  O(n)
// Space: O(1)
// pub struct Solution3 {}
// impl Solution3 {
//     pub fn missing_number(nums: Vec<i32>) -> i32 {
//         0
//     }
// }

// Time:  O(n)
// Space: O(1)
pub struct Solution4 {}
impl Solution4 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        ((0..nums.len() + 1).fold(0, |acc, i| acc + (i as i32))) - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(Solution1::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(
            Solution1::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );

        // assert_eq!(Solution2::missing_number(vec![3, 0, 1]), 2);
        // assert_eq!(
        //     Solution2::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
        //     8
        // );

        // assert_eq!(Solution3::missing_number(vec![3, 0, 1]), 2);
        // assert_eq!(
        //     Solution2::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
        //     8
        // );

        assert_eq!(Solution4::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(
            Solution4::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }
}
