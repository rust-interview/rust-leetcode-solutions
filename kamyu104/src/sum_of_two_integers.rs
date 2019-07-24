// Time:  O(1)
// Space: O(1)

pub struct Solution1 {}
impl Solution1 {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carry = a & b;
            a ^= b;
            b = carry << 1;
        }
        a
    }
}

// pub struct Solution2 {}
// impl Solution2 {
//     pub fn get_sum(a: i32, b: i32) -> i32 {
//         let bit_length = 32;
//         let neg_bit = (1 << bit_length) >> 1;
//         let mask = !(!0 << bit_length);

//         let mut ma = a & mask;
//         if a & neg_bit != 0 {
//             ma = a | !mask;
//         }

//         let mut mb = b & mask;
//         if b & neg_bit != 0 {
//             mb = b | !mask;
//         }

//         while mb != 0 {
//             let carry = ma & mb;
//             ma ^= mb;
//             if ma & neg_bit != 0 {
//                 ma = ma | !mask;
//             } else {
//                 ma = ma & mask;
//             }
//             mb = carry << 1;
//             if mb & neg_bit != 0 {
//                 mb = mb | !mask;
//             } else {
//                 mb = mb & mask;
//             }
//         }
//         a
//     }
// }

// pub struct Solution3 {}
// impl Solution3 {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_sum() {
        assert_eq!(Solution1::get_sum(1, 2), 3);
        assert_eq!(Solution1::get_sum(-2, 3), 1);

        // assert_eq!(Solution2::get_sum(1, 2), 3);
        // assert_eq!(Solution2::get_sum(-2, 3), 1);
    }
}
