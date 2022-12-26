pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Ids {
    ids: [usize; 26],
    length: usize,
}

impl Ids {
    fn get(&mut self, c: u8) -> usize {
        let id = &mut self.ids[usize::from(c) - usize::from(b'A')];

        if *id == 0 {
            self.length += 1;
            *id = self.length;
        }

        *id - 1
    }
}

struct Context {
    factors: [i32; 10],
    not_zero: u16,
    required: usize,
}

impl Solution {
    fn permutations(context: &Context, used: u16, used_count: usize, sum: i32) -> bool {
        if used_count < context.required {
            let mut available = used ^ ((1 << 10) - 1);

            if context.not_zero & (1 << used_count) != 0 {
                available &= !1;
            }

            let factor = context.factors[used_count];

            while available != 0 {
                let digit = available.trailing_zeros();

                if Self::permutations(
                    context,
                    used | (1 << digit),
                    used_count + 1,
                    sum + factor * (digit as i32),
                ) {
                    return true;
                }

                available &= available - 1;
            }

            false
        } else {
            sum == 0
        }
    }

    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let mut not_zero = 0;
        let mut factors = [0; 10];
        let mut ids = Ids::default();

        // Process `words`.

        for word in words {
            if let [first, _, ..] = *word.as_bytes() {
                not_zero |= 1 << ids.get(first);
            }

            let mut base = 1;

            for c in word.bytes().rev() {
                factors[ids.get(c)] += base;
                base *= 10;
            }
        }

        // Process `result`.

        if let [first, _, ..] = *result.as_bytes() {
            not_zero |= 1 << ids.get(first);
        }

        let mut base = 1;

        for c in result.bytes().rev() {
            factors[ids.get(c)] -= base;
            base *= 10;
        }

        drop(result);

        // Search all permutations.

        Self::permutations(
            &Context {
                factors,
                not_zero,
                required: ids.length,
            },
            0,
            0,
            0,
        )
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
