use std::collections::BTreeSet;

pub struct Solution {}
impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        (0..12).for_each(|h| {
            (0..60).for_each(|m| {
                if (Solution::bit_count(h) + Solution::bit_count(m)) == num {
                    let elem = format!("{}:{:02}", h, m);
                    result.push(elem);
                }
            });
        });
        result
    }

    fn bit_count(mut bits: i32) -> i32 {
        let mut count: i32 = 0;
        while bits != 0 {
            count += 1;
            bits &= bits - 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_binary_watch() {
        assert_eq!(
            Solution::read_binary_watch(1)
                .iter()
                .collect::<BTreeSet<&String>>(),
            vec!["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
                .iter()
                .map(|&x| String::from(x))
                .collect::<BTreeSet<String>>()
        );
        assert_eq!(
            Solution::read_binary_watch(1)
                .iter()
                .collect::<BTreeSet<&String>>(),
            vec!["1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"]
                .iter()
                .map(|&x| x.into())
                .collect::<BTreeSet<String>>()
        );
    }
}
