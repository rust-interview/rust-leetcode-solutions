// Time:  O(n) ~ O(n^2)
// Space: O(n)
// Counting Sort + Pruning + Bit Manipulation
pub struct Solution1 {}
impl Solution1 {
    pub fn max_product(words: Vec<String>) -> i32 {
        0
    }
}


// Time:  O(nlogn) ~ O(n^2)
// Space: O(n)
// Sorting + Pruning + Bit Manipulation
pub struct Solution2 {}
impl Solution2 {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut ws = words.clone();
        ws.sort_by(|a, b| a.len().cmp(&b.len()));
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_max_product() {
        // assert_eq!(Solution1::max_product(vec!["abcw","baz","foo","bar","xtfn","abcdef"].iter().map(|&x| String::from(x)).collect()), 16);
        // assert_eq!(Solution1::max_product(vec!["a","ab","abc","d","cd","bcd","abcd"].iter().map(|&x| String::from(x)).collect()), 4);
        // assert_eq!(Solution1::max_product(vec!["a","aa","aaa","aaaa"].iter().map(|&x| String::from(x)).collect()), 0);

        // assert_eq!(Solution2::max_product(vec!["abcw","baz","foo","bar","xtfn","abcdef"].iter().map(|&x| String::from(x)).collect()), 16);
        // assert_eq!(Solution2::max_product(vec!["a","ab","abc","d","cd","bcd","abcd"].iter().map(|&x| String::from(x)).collect()), 4);
        // assert_eq!(Solution2::max_product(vec!["a","aa","aaa","aaaa"].iter().map(|&x| String::from(x)).collect()), 0);
    }
}
