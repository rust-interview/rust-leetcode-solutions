pub struct Solution1 {}
pub struct Solution2 {}
pub struct Solution3 {}
pub struct Solution4 {}

impl Solution1 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut one, mut two): (i32, i32) = (0, 0);
        for num in nums.iter() {
            let new_one = (!num & one) | (num & !one & !two);
            let new_two = (!num & two) | (num & one);
            one = new_one;
            two = new_two;
        }
        one
    }
}

impl Solution2 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut one, mut two, mut carry): (i32, i32, i32) = (0, 0, 0);
        for num in nums.iter() {
            two |= one & num;
            one ^= num;
            carry = one & two;
            one &= !carry;
            two &= !carry;
        }
        one
    }
}

impl Solution3 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut one, mut two, mut carry): (i32, i32, i32) = (0, 0, 0);
        for num in nums.iter() {
            two |= one & num;
            one ^= num;
            carry = one & two;
            one &= !carry;
            two &= !carry;
        }
        one
    }
}

impl Solution4 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut one, mut two, mut carry): (i32, i32, i32) = (0, 0, 0);
        for num in nums.iter() {
            two |= one & num;
            one ^= num;
            carry = one & two;
            one &= !carry;
            two &= !carry;
        }
        one
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Solution1::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution1::single_number(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution1::single_number(vec![0, 0, 0, 1, 1, 1, 5]), 5);

        assert_eq!(Solution2::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution2::single_number(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution2::single_number(vec![0, 0, 0, 1, 1, 1, 5]), 5);

        assert_eq!(Solution3::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution3::single_number(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution3::single_number(vec![0, 0, 0, 1, 1, 1, 5]), 5);

        assert_eq!(Solution4::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution4::single_number(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution4::single_number(vec![0, 0, 0, 1, 1, 1, 5]), 5);
    }
}
