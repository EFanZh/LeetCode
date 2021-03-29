pub struct Solution;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct State(u32);

impl State {
    fn get_choosable_integers(self) -> impl Iterator<Item = u32> {
        let mut choosable_integers = self.0 & ((1 << 20) - 1);

        iter::from_fn(move || {
            if choosable_integers == 0 {
                None
            } else {
                let zeros = choosable_integers.trailing_zeros();

                choosable_integers &= choosable_integers - 1;

                Some(zeros + 1)
            }
        })
    }

    fn can_i_win_this_round(self) -> bool {
        let max_choosable_integer = 32 - (self.0 & ((1 << 20) - 1)).leading_zeros();
        let desired_total = self.0 >> 20;

        max_choosable_integer >= desired_total
    }

    fn remove_integer(self, integer: u32) -> Self {
        Self((self.0 & !(1 << (integer - 1))) - (integer << 20))
    }
}

impl Solution {
    fn helper(state: State, cache: &mut HashMap<State, bool>) -> bool {
        if let Some(&result) = cache.get(&state) {
            result
        } else if state.can_i_win_this_round() {
            true
        } else {
            let result = state
                .get_choosable_integers()
                .any(|choose| !Self::helper(state.remove_integer(choose), cache));

            cache.insert(state, result);

            result
        }
    }

    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        let total = (1 + max_choosable_integer) * max_choosable_integer / 2;

        match total.cmp(&desired_total) {
            Ordering::Less => false,
            Ordering::Equal => max_choosable_integer % 2 != 0,
            Ordering::Greater => {
                let choosable_integers = (1 << max_choosable_integer) - 1;

                Self::helper(
                    State((choosable_integers | (desired_total << 20)) as _),
                    &mut HashMap::new(),
                )
            }
        }
    }
}

impl super::Solution for Solution {
    fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        Self::can_i_win(max_choosable_integer, desired_total)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
