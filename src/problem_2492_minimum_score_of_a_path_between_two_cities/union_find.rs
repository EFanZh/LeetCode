pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::{mem, ptr};

type State = (Cell<u32>, Cell<u32>, Cell<u32>);

impl Solution {
    fn get_node(states: &[State], state: &State) -> u32 {
        ((ptr::from_ref(state) as usize - states.as_ptr() as usize) / mem::size_of::<State>()) as _
    }

    fn find_root<'a>(states: &'a [State], state: &'a State) -> &'a State {
        let parent = state.0.get();

        states.get(parent as usize).map_or(state, |parent_state| {
            let root = Self::find_root(states, parent_state);

            state.0.set(Self::get_node(states, root));

            root
        })
    }

    fn union(states: &[State], left_state: &State, right_state: &State, distance: u32) {
        let left_root = Self::find_root(states, left_state);
        let right_root = Self::find_root(states, right_state);

        if ptr::eq(left_root, right_root) {
            left_root.2.set(left_root.2.get().min(distance));
        } else {
            let left_rank = left_root.1.get();
            let right_rank = right_root.1.get();

            let (child, parent) = if left_rank < right_rank {
                (left_root, right_root)
            } else {
                if left_rank == right_rank {
                    left_root.1.set(left_rank + 1);
                }

                (right_root, left_root)
            };

            child.0.set(Self::get_node(states, parent));
            parent.2.set(parent.2.get().min(child.2.get()).min(distance));
        }
    }

    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let union_find = vec![(Cell::new(u32::MAX), Cell::new(0), Cell::new(u32::MAX)); n as u32 as usize];

        for road in roads {
            let [mut a, mut b, distance] = <[_; 3]>::map(road.try_into().ok().unwrap(), |x| x as u32);

            a -= 1;
            b -= 1;

            Self::union(&union_find, &union_find[a as usize], &union_find[b as usize], distance);
        }

        Self::find_root(&union_find, &union_find[0]).2.get() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        Self::min_score(n, roads)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
