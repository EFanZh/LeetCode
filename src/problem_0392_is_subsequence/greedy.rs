pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter_2 = t.bytes();

        for left in s.bytes() {
            if iter_2.all(|right| right != left) {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_subsequence(s: String, t: String) -> bool {
        Self::is_subsequence(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
