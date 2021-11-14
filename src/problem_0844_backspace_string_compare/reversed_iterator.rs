pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn reversed_iterator(s: &str) -> impl Iterator<Item = u8> + '_ {
        let mut base = s.bytes().rev();
        let mut skip = 0;

        iter::from_fn(move || {
            for c in &mut base {
                if c == b'#' {
                    skip += 1;
                } else if skip == 0 {
                    return Some(c);
                } else {
                    skip -= 1;
                }
            }

            None
        })
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::reversed_iterator(&s).eq(Self::reversed_iterator(&t))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn backspace_compare(s: String, t: String) -> bool {
        Self::backspace_compare(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
