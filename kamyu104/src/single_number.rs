// Time:  O(n)
// Space: O(1)

#[allow(dead_code)]
pub fn single_number(a: i32, b: i32) -> i32 {
    // only print when a test fails
    println!("{}", a + b);
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(1, 2), 3);
        assert_ne!(single_number(1, 2), 4);
    }
}