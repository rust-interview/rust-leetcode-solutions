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
        let mut sorted_words = words.clone();
        sorted_words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut bits: Vec<i32> = vec![0; words.len()];
        for i in 0..sorted_words.len() {
            for c in sorted_words[i].chars() {
                bits[i] |= 1 << (c as u32 - 'a' as u32);
            }
        }
        let mut max_product: i32 = 0;
        let mut i: usize = 0;
        while i + 1 < sorted_words.len() && sorted_words[i].len().pow(2) as i32 > max_product {
            let mut j: usize = i + 1;
            while j < sorted_words.len()
                && (sorted_words[i].len() * sorted_words[j].len()) as i32 > max_product
            {
                if bits[i] & bits[j] == 0 {
                    max_product = (sorted_words[i].len() * sorted_words[j].len()) as i32;
                }
                j += 1;
            }

            i += 1;
        }
        max_product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(
            Solution2::max_product(
                vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
                    .iter()
                    .map(|&x| String::from(x))
                    .collect()
            ),
            16
        );
        assert_eq!(
            Solution2::max_product(
                vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"]
                    .iter()
                    .map(|&x| String::from(x))
                    .collect()
            ),
            4
        );
        assert_eq!(
            Solution2::max_product(
                vec!["a", "aa", "aaa", "aaaa"]
                    .iter()
                    .map(|&x| String::from(x))
                    .collect()
            ),
            0
        );

        // assert_eq!(Solution2::max_product(vec!["abcw","baz","foo","bar","xtfn","abcdef"].iter().map(|&x| String::from(x)).collect()), 16);
        // assert_eq!(Solution2::max_product(vec!["a","ab","abc","d","cd","bcd","abcd"].iter().map(|&x| String::from(x)).collect()), 4);
        // assert_eq!(Solution2::max_product(vec!["a","aa","aaa","aaaa"].iter().map(|&x| String::from(x)).collect()), 0);
    }
}
