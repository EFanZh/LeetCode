pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroUsize;

impl Solution {
    fn assign_add(value: &mut usize, x: usize, n: NonZeroUsize) {
        let candidate = *value + x;

        *value = candidate.checked_sub(n.get()).unwrap_or(candidate);
    }

    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let n = NonZeroUsize::new(n as u32 as _).unwrap();
        let k = k as u32 as usize;

        assert!(k <= n.get());

        let states = &mut [false; 50][..n.get()];
        let mut i = 0;
        let mut step = k;

        while let state @ false = &mut states[i] {
            *state = true;

            Self::assign_add(&mut i, step, n);
            Self::assign_add(&mut step, k, n);
        }

        (1..)
            .zip(states)
            .filter_map(|(i, state)| (!*state).then_some(i))
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        Self::circular_game_losers(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
