pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::ControlFlow;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = [0_u32; 26];

        for c in word.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        drop(word);

        counts.sort_unstable();

        let mut result = 0;

        _ = (8..)
            .zip(counts.iter().rev().copied())
            .try_for_each(|(assignment, count)| {
                if count == 0 {
                    return ControlFlow::Break(());
                }

                result += (assignment / 8) * count;

                ControlFlow::Continue(())
            });

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_pushes(word: String) -> i32 {
        Self::minimum_pushes(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
