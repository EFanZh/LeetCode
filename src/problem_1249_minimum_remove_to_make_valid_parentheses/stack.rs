pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut s = s.into_bytes();
        let mut stack = Vec::new();

        for c in &mut s {
            match c {
                b'(' => stack.push(c),
                b')' => {
                    if stack.pop().is_none() {
                        *c = 0;
                    }
                }
                _ => {}
            }
        }

        for c in stack {
            *c = 0;
        }

        s.retain(|&c| c != 0);

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_remove_to_make_valid(s: String) -> String {
        Self::min_remove_to_make_valid(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
