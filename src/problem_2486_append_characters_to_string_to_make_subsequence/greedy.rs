pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut s_iter = s.bytes();
        let mut t_iter = t.bytes();

        while let Some(expected) = t_iter.next() {
            if s_iter.all(|actual| actual != expected) {
                return (t_iter.count() + 1) as _;
            }
        }

        0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn append_characters(s: String, t: String) -> i32 {
        Self::append_characters(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
