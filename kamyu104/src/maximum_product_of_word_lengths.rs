// Time:  O(n) ~ O(n^2)
// Space: O(n)
// Counting Sort + Pruning + Bit Manipulation
// https://github.com/kamyu104/LeetCode-Solutions/blob/master/C++/maximum-product-of-word-lengths.cpp#L1-L40
// pub struct Solution1 {}
// impl Solution1 {
//     pub fn max_product(words: Vec<String>) -> i32 {
//         0
//     }
// }

// Time:  O(nlogn) ~ O(n^2)
// Space: O(n)
// Sorting + Pruning + Bit Manipulation
pub struct Solution2 {}
impl Solution2 {
    pub fn max_product(mut words: Vec<String>) -> i32 {
        words.sort_by(|a, b| b.len().cmp(&a.len()));
        let mut bits: Vec<i32> = vec![0; words.len()];
        for i in 0..words.len() {
            for c in words[i].chars() {
                bits[i] |= 1 << (c as u32 - 'a' as u32);
            }
        }
        let mut max_product: i32 = 0;
        let mut i: usize = 0;
        while i + 1 < words.len() && words[i].len().pow(2) as i32 > max_product {
            let mut j: usize = i + 1;
            while j < words.len() && (words[i].len() * words[j].len()) as i32 > max_product {
                if bits[i] & bits[j] == 0 {
                    max_product = (words[i].len() * words[j].len()) as i32;
                }
                j += 1;
            }
            i += 1;
        }
        max_product
    }
}

// # Time:  O(n) ~ O(n^2)
// # Space: O(n)
// https://github.com/kamyu104/LeetCode-Solutions/blob/master/Python/maximum-product-of-word-lengths.py#L1-L36
// pub struct Solution3 {}
// impl Solution3 {
//     pub fn max_product(words: Vec<String>) -> i32 {
//         0
//     }
// }

// # Time:  O(nlogn) ~ O(n^2)
// # Space: O(n)
// # Sorting + Pruning + Bit Manipulation
// https://github.com/kamyu104/LeetCode-Solutions/blob/master/Python/maximum-product-of-word-lengths.py#L38-L62
// pub struct Solution4 {}
// impl Solution4 {
//     pub fn max_product(words: Vec<String>) -> i32 {
//         0
//     }
// }

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
    }
}
