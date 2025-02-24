pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::num::NonZeroU32;

impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let space = NonZeroU32::new(space as _).unwrap();
        let buckets = space.get() as usize;
        let mut state = (0, 0);

        let update_state = |&new_state: &(u32, u32)| match new_state.0.cmp(&state.0) {
            Ordering::Less => {}
            Ordering::Equal => state.1 = state.1.min(new_state.1),
            Ordering::Greater => state = new_state,
        };

        if buckets <= nums.len() {
            let mut groups = vec![(0_u32, 0_u32); buckets].into_boxed_slice();

            for num in nums {
                let num = num as u32;
                let state = &mut groups[(num % space) as usize];

                *state = if state.0 == 0 {
                    (1, num)
                } else {
                    (state.0 + 1, state.1.min(num))
                };
            }

            groups.iter().for_each(update_state);
        } else {
            let mut groups = HashMap::<u32, (u32, u32)>::new();

            for num in nums {
                let num = num as u32;

                match groups.entry(num % space) {
                    Entry::Occupied(occupied_entry) => {
                        let state = occupied_entry.into_mut();

                        state.0 += 1;
                        state.1 = state.1.min(num);
                    }
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert((1, num));
                    }
                }
            }

            groups.values().for_each(update_state);
        }

        state.1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        Self::destroy_targets(nums, space)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
