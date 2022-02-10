pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn iter_chunks(s: &str) -> impl Iterator<Item = (u8, usize)> + '_ {
        let mut state = s.as_bytes().split_first().map(|(&first, rest)| (first, rest));

        iter::from_fn(move || {
            state.map(|(first, rest)| {
                let i = if let Some((i, c)) = rest.iter().copied().enumerate().find(|&(_, c)| c != first) {
                    state = Some((c, &rest[i + 1..]));

                    i
                } else {
                    state = None;

                    rest.len()
                };

                (first, i + 1)
            })
        })
    }

    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut typed_iter = Self::iter_chunks(&typed);

        for (c_1, length_1) in Self::iter_chunks(&name) {
            if let Some((c_2, length_2)) = typed_iter.next() {
                if c_2 != c_1 || length_2 < length_1 {
                    return false;
                }
            } else {
                return false;
            }
        }

        typed_iter.next().is_none()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_long_pressed_name(name: String, typed: String) -> bool {
        Self::is_long_pressed_name(name, typed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
