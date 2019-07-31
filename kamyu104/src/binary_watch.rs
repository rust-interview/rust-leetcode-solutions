pub struct Solution {}
impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        vec![]
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
