pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut result = 0;
        let mut i = 0;

        while let Some(&c) = s.as_bytes().get(i) {
            if c == b'O' {
                i += 1;
            } else {
                result += 1;
                i += 3;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_moves(s: String) -> i32 {
        Self::minimum_moves(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
