// Time:  O(n)
// Space: O(1)
pub struct Solution {}
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut count: i32 = 0;
        for c in data {
            match count {
                0 => {
                    if (c >> 5) == 0b110 {
                        count = 1;
                    } else if (c >> 4) == 0b1110 {
                        count = 2;
                    } else if (c >> 3) == 0b11110 {
                        count = 3;
                    } else if c >> 7 != 0 {
                        return false;
                    }
                }
                _ => {
                    if (c >> 6) != 0b10 {
                        return false;
                    }
                    count -= 1;
                }
            }
        }
        count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_utf8() {
        assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
        assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
    }
}
