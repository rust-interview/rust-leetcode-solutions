pub struct Solution {}

impl Solution {
    pub fn single_number1(nums: Vec<i32>) -> i32 {
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Solution::single_number1(vec![2,2,3,2]), 3);
        assert_eq!(Solution::single_number1(vec![0,1,0,1,0,1,99]), 99);
        assert_eq!(Solution::single_number1(vec![0, 0, 0, 1, 1, 1, 5]), 5);
    }
}
