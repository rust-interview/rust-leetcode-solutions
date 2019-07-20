pub struct Solution {}
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_max_product!(Solution::max_product(vec!["abcw","baz","foo","bar","xtfn","abcdef"]), 16);
        assert_max_product!(Solution::max_product(vec!["a","ab","abc","d","cd","bcd","abcd"]), 4);
        assert_max_product!(Solution::max_product(vec!["a","aa","aaa","aaaa"]), 0);
    }
}
