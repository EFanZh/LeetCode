pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let mut iter = str1.bytes();

        str2.bytes()
            .all(|x| iter.any(|y| matches!(x.wrapping_sub(y), 0 | 1 | 231)))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_make_subsequence(str1: String, str2: String) -> bool {
        Self::can_make_subsequence(str1, str2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
