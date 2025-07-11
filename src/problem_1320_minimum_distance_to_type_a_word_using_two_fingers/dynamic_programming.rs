pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn distance(from: u8, to: u8) -> u16 {
        if from == 26 {
            0
        } else {
            let from_row = from / 6;
            let from_column = from % 6;
            let to_row = to / 6;
            let to_column = to % 6;

            u16::from(from_row.abs_diff(to_row) + from_column.abs_diff(to_column))
        }
    }

    pub fn minimum_distance(word: String) -> i32 {
        let mut cache = [u16::MAX; 27];
        let mut prev = 26;

        cache[26] = 0;

        for c in word.into_bytes() {
            let c = c - b'A';
            let move_right_cost = Self::distance(prev, c);
            let mut move_left_cost = u16::MAX;

            for (i, target) in (0..).zip(&mut cache) {
                move_left_cost = move_left_cost.min(target.saturating_add(Self::distance(i, c)));
                *target = target.saturating_add(move_right_cost);
            }

            let slot = &mut cache[usize::from(prev)];

            *slot = (*slot).min(move_left_cost);
            prev = c;
        }

        i32::from(cache.iter().copied().min().unwrap())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_distance(word: String) -> i32 {
        Self::minimum_distance(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
