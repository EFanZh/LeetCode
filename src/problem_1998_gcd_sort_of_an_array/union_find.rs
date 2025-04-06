pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::num::NonZeroU32;
use std::ptr;

type State = (Cell<u32>, Cell<u32>);

impl Solution {
    fn get_min_factors(max: u32) -> Box<[NonZeroU32]> {
        let mut result = vec![NonZeroU32::MAX; (max + 1) as usize].into_boxed_slice();
        let middle = max.isqrt();

        for i in 2..=middle {
            let i = NonZeroU32::new(i).unwrap();
            let i_usize = i.get() as usize;

            if result[i_usize] == NonZeroU32::MAX {
                let mut j = (i.get() * i.get()) as usize;

                while let Some(state) = result.get_mut(j) {
                    if *state == NonZeroU32::MAX {
                        *state = i;
                    }

                    j += i_usize;
                }
            }
        }

        result
    }

    fn get_prime_factors(min_factors: &[NonZeroU32], mut num: u32, mut f: impl FnMut(u32)) {
        let mut min_factor = min_factors[num as usize];

        while min_factor != NonZeroU32::MAX {
            f(min_factor.get());

            loop {
                num /= min_factor;

                let next_min_factor = min_factors[num as usize];

                if next_min_factor != min_factor {
                    min_factor = next_min_factor;

                    break;
                }
            }
        }

        f(num);
    }

    fn get_node(union_find: &[State], state: &State) -> u32 {
        ((ptr::from_ref(state) as usize - union_find.as_ptr() as usize) / size_of::<State>()) as _
    }

    fn find_root_helper<'a>(union_find: &'a [State], state: &'a State) -> &'a State {
        let parent = state.0.get();

        union_find.get(parent as usize).map_or(state, |parent_state| {
            let root = Self::find_root_helper(union_find, parent_state);

            state.0.set(Self::get_node(union_find, root));

            root
        })
    }

    fn find_root(union_find: &[State], node: u32) -> &State {
        let state = &union_find[node as usize];

        Self::find_root_helper(union_find, state)
    }

    fn union(union_find: &[State], left: u32, right: u32) {
        let left_root = Self::find_root(union_find, left);
        let right_root = Self::find_root(union_find, right);

        if !ptr::eq(left_root, right_root) {
            let left_rank = left_root.1.get();
            let right_rank = right_root.1.get();

            if left_rank < right_rank {
                left_root.0.set(Self::get_node(union_find, right_root));
            } else {
                if left_rank == right_rank {
                    left_root.1.set(left_rank + 1);
                }

                right_root.0.set(Self::get_node(union_find, left_root));
            }
        }
    }

    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        type StableHasher = BuildHasherDefault<DefaultHasher>;

        let nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let nums_set = nums.iter().copied().collect::<HashSet<_, StableHasher>>();
        let max_num = nums_set.iter().copied().max().unwrap();
        let min_factors = Self::get_min_factors(max_num);
        let mut factors = HashMap::<_, Vec<_>, _>::with_hasher(StableHasher::default());

        for num in nums_set {
            Self::get_prime_factors(&min_factors, num, |factor| match factors.entry(factor) {
                Entry::Occupied(entry) => entry.into_mut().push(num),
                Entry::Vacant(entry) => {
                    entry.insert(vec![num]);
                }
            });
        }

        let union_find = vec![(Cell::new(u32::MAX), Cell::new(0)); (max_num + 1) as _].into_boxed_slice();

        for group in factors.into_values() {
            let mut iter = group.iter().copied();
            let first = iter.next().unwrap();

            for num in iter {
                Self::union(&union_find, first, num);
            }
        }

        let mut nums_2 = nums.clone();

        nums_2.sort_unstable();

        nums.iter().zip(&nums_2).all(|(&lhs, &rhs)| {
            let left_root = Self::find_root(&union_find, lhs);
            let right_root = Self::find_root(&union_find, rhs);

            ptr::eq(left_root, right_root)
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn gcd_sort(nums: Vec<i32>) -> bool {
        Self::gcd_sort(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
