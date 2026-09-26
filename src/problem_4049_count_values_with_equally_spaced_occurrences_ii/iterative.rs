pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

enum State {
    One(u32),
    Two(u32, u32),
    Valid(u32, u32),
    Bad,
}

impl Solution {
    pub fn count_special_integers(nums: Vec<i32>) -> i32 {
        let mut states = HashMap::new();

        (0..).zip(nums).for_each(|(i, num)| match states.entry(num) {
            Entry::Occupied(occupied_entry) => {
                let state = occupied_entry.into_mut();

                *state = match *state {
                    State::One(prev) => State::Two(i, i - prev),
                    State::Two(prev, step) | State::Valid(prev, step) => {
                        if i == prev + step {
                            State::Valid(i, step)
                        } else {
                            State::Bad
                        }
                    }
                    State::Bad => return,
                };
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(State::One(i));
            }
        });

        states
            .values()
            .filter(|state| matches!(state, State::Valid(..)))
            .count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_special_integers(nums: Vec<i32>) -> i32 {
        Self::count_special_integers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
