pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;

impl Solution {
    fn get_choosable_integers(state: u32) -> impl Iterator<Item = u32> {
        let mut choosable_integers = state & ((1 << 20) - 1);

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

    fn can_i_win_this_round(state: u32) -> bool {
        let max_choosable_integer = 32 - (state & ((1 << 20) - 1)).leading_zeros();
        let desired_total = state >> 20;

        max_choosable_integer >= desired_total
    }

    fn remove_integer(state: u32, integer: u32) -> u32 {
        (state & !(1 << (integer - 1))) - (integer << 20)
    }

    fn helper(state: u32, cache: &mut HashMap<u32, bool>) -> bool {
        if let Some(&result) = cache.get(&state) {
            result
        } else if Self::can_i_win_this_round(state) {
            true
        } else {
            let result = Self::get_choosable_integers(state)
                .any(|choose| !Self::helper(Self::remove_integer(state, choose), cache));

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

                Self::helper((choosable_integers | (desired_total << 20)) as _, &mut HashMap::new())
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
