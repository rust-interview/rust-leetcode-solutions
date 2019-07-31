use std::collections::HashSet;

// Time:  O(n)
// Space: O(1)

// pub struct Solution1 {}
// impl Solution1 {
//     pub fn find_the_difference(s: String, t: String) -> char {
//         'e'
//     }
// }

// pub struct Solution2 {}
// impl Solution2 {
//     pub fn find_the_difference(s: String, t: String) -> char {
//         'e'
//     }
// }

// https://github.com/kamyu104/LeetCode-Solutions/blob/master/Python/find-the-difference.py#L18
pub struct Solution3 {}
impl Solution3 {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s_chars: HashSet<char> = s.chars().collect();
        let mut t_chars: HashSet<char> = t.chars().collect();
        s_chars.iter().for_each(|c| {
            t_chars.remove(c);
        });
        t_chars.into_iter().collect::<Vec<char>>()[0]
    }
}

// pub struct Solution4 {}
// impl Solution4 {
//     pub fn find_the_difference(s: String, t: String) -> char {
//         'e'
//     }
// }

// pub struct Solution5 {}
// impl Solution5 {
//     pub fn find_the_difference(s: String, t: String) -> char {
//         'e'
//     }
// }

// pub struct Solution6 {}
// impl Solution6 {
//     pub fn find_the_difference(s: String, t: String) -> char {
//         'e'
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_difference() {
        assert_eq!(
            Solution3::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }
}
