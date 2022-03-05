pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper_1(range: &mut [i32], step: usize, length: usize, iter: &mut impl Iterator<Item = i32>) {
        let next_step = step * 2;

        for (target, source) in range.iter_mut().step_by(next_step).zip(iter.by_ref()) {
            *target = source;
        }

        if let Some(rest) = range.get_mut(step..) {
            let next_length = length / 2;

            if next_length != 0 {
                if length % 2 == 0 {
                    Self::helper_1(rest, next_step, next_length, iter);
                } else {
                    Self::helper_2(rest, next_step, next_length, iter);
                }
            }
        }
    }

    fn helper_2(range: &mut [i32], step: usize, length: usize, iter: &mut impl Iterator<Item = i32>) {
        let next_step = step * 2;

        if let Some(range) = range.get_mut(step..) {
            for (target, source) in range.iter_mut().step_by(next_step).zip(iter.by_ref()) {
                *target = source;
            }
        }

        let next_length = (length + 1) / 2;

        if length % 2 == 0 {
            Self::helper_2(range, next_step, next_length, iter);
        } else {
            Self::helper_1(range, next_step, next_length, iter);
        }
    }

    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;

        deck.sort_unstable();

        let n = deck.len();
        let mut result = vec![0; n];

        Self::helper_1(&mut result, 1, n, &mut deck.into_iter());

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        Self::deck_revealed_increasing(deck)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
