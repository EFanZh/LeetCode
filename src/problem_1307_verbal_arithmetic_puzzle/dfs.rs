pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context {
    words: Vec<String>,
    result: Vec<u8>,
    not_zero: u32,
    map: [u8; 26],
}

impl Solution {
    fn dfs(context: &mut Context, digits: usize, word_index: usize, used: u16, sum: u32) -> bool {
        let base = u32::pow(10, digits as u32 - 1);

        if let Some(word) = context.words.get(word_index).map(String::as_bytes) {
            let digit_index = word.len().wrapping_sub(digits);

            if let Some(&c) = word.get(digit_index) {
                let map_index = usize::from(c) - usize::from(b'A');
                let digit_slot = &mut context.map[map_index];

                if *digit_slot == u8::MAX {
                    let mut available = used ^ ((1 << 10) - 1);

                    if context.not_zero & (1 << map_index) != 0 {
                        available &= !1;
                    }

                    while available != 0 {
                        let digit = available.trailing_zeros();

                        context.map[map_index] = digit as _;

                        if Self::dfs(context, digits, word_index + 1, used | (1 << digit), sum + base * digit) {
                            return true;
                        }

                        available &= available - 1;
                    }

                    context.map[map_index] = u8::MAX;

                    false
                } else {
                    let digit = *digit_slot;

                    Self::dfs(context, digits, word_index + 1, used, sum + base * u32::from(digit))
                }
            } else {
                Self::dfs(context, digits, word_index + 1, used, sum)
            }
        } else {
            let expected = (sum / base) as u8 % 10;
            let map_index = usize::from(context.result[context.result.len() - digits]) - usize::from(b'A');
            let actual = &mut context.map[map_index];

            if expected == 0 && (context.not_zero & (1 << map_index) != 0) {
                false
            } else if digits == context.result.len() {
                if sum < base * 10 {
                    if *actual == u8::MAX {
                        used & (1 << expected) == 0
                    } else {
                        *actual == expected
                    }
                } else {
                    false
                }
            } else if *actual == u8::MAX {
                let probe = 1 << expected;

                if used & probe == 0 {
                    *actual = expected;

                    let result = Self::dfs(context, digits + 1, 0, used | probe, sum);

                    context.map[map_index] = u8::MAX;

                    result
                } else {
                    false
                }
            } else if *actual == expected {
                Self::dfs(context, digits + 1, 0, used, sum)
            } else {
                false
            }
        }
    }

    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let n = result.len();
        let mut not_zero = 0;

        for word in &words {
            if word.len() <= n {
                if let [first, _, ..] = *word.as_bytes() {
                    not_zero |= 1 << (first - b'A');
                }
            } else {
                return false;
            }
        }

        if let [first, _, ..] = *result.as_bytes() {
            not_zero |= 1 << (first - b'A');
        }

        let mut context = Context {
            words,
            result: result.into_bytes(),
            not_zero,
            map: [u8::MAX; 26],
        };

        Self::dfs(&mut context, 1, 0, 0, 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_solvable(words: Vec<String>, result: String) -> bool {
        Self::is_solvable(words, result)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
