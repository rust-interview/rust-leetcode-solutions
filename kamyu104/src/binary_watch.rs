pub struct Solution {}
impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        // vec![]
        let result: Vec<String> = Vec::new();
        /*for (int h = 0; h < 12; ++h) {
            for (int m = 0; m < 60; ++m) {
                if (bit_count(h) + bit_count(m) == num) {
                    const auto hour = to_string(h);
                    const auto minute = m < 10 ? "0" + to_string(m) : to_string(m);
                    result.emplace_back(hour + ":" + minute);
                }
            }
        }*/
        result
    }

    fn bit_count(bits: i32) -> i32 {
        let mut count: i32 = 0;
        // for (; bits; bits &= bits - 1) {
        //     ++count;
        // }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_binary_watch() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec!["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
                .iter()
                .map(|&x| String::from(x))
                .collect::<Vec<String>>()
        );
        assert_eq!(
            Solution::read_binary_watch(1),
            vec!["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
                .iter()
                .map(|&x| x.into())
                .collect::<Vec<String>>()
        );
    }
}
