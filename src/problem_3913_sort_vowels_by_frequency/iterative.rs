pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut states = b"aeiou".map(|c| (0_u32, u32::MAX, c));

        (0..).zip(&s).for_each(|(i, c)| {
            let state = &mut states[match c {
                b'a' => 0,
                b'e' => 1,
                b'i' => 2,
                b'o' => 3,
                b'u' => 4,
                _ => return,
            }];

            state.0 += 1;

            if state.1 == u32::MAX {
                state.1 = i;
            }
        });

        states.sort_unstable_by_key(|state| (Reverse(state.0), state.1));

        let mut cursor = 0;

        for c in &mut s {
            if matches!(c, b'a' | b'e' | b'i' | b'o' | b'u') {
                let state = loop {
                    let state = &mut states[cursor];

                    if state.0 == 0 {
                        cursor += 1;
                    } else {
                        break state;
                    }
                };

                state.0 -= 1;
                *c = state.2;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_vowels(s: String) -> String {
        Self::sort_vowels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
