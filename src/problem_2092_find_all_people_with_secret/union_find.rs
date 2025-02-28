pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::ptr;

type State = (Cell<u32>, Cell<u32>);

impl Solution {
    fn get_node(union_find: &[State], state: &State) -> u32 {
        ((ptr::from_ref(state) as usize - union_find.as_ptr() as usize) / size_of::<State>()) as _
    }

    fn find_root<'a>(union_find: &'a [State], state: &'a State) -> &'a State {
        let parent = state.0.get();

        union_find.get(parent as usize).map_or(state, |parent_state| {
            let root = Self::find_root(union_find, parent_state);

            state.0.set(Self::get_node(union_find, root));

            root
        })
    }

    fn union(union_find: &[State], left: &State, right: &State) {
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

    fn broadcast_in_time_frame(union_find: &[State], time_frame: &mut Vec<(&State, &State)>) {
        for &(x, y) in &*time_frame {
            Self::union(union_find, x, y);
        }

        let zero_root = Self::find_root(union_find, &union_find[0]);

        while let Some(item) = time_frame.pop() {
            if !ptr::eq(Self::find_root(union_find, item.0), zero_root) {
                for node in <[_; 2]>::from(item) {
                    node.0.set(u32::MAX);
                    node.1.set(0);
                }
            }
        }
    }

    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let n = n as u32 as usize;

        let mut meetings = meetings
            .iter()
            .map(|meeting| <&[_; 3]>::try_from(meeting.as_slice()).ok().unwrap())
            .collect::<Vec<_>>();

        let first_person = first_person as u32;

        meetings.sort_unstable_by_key(|&&[_, _, time]| time as u32);

        let union_find = vec![(Cell::new(u32::MAX), Cell::new(0)); n].into_boxed_slice();
        let mut prev_time = 0;
        let mut time_frame = vec![(&union_find[0], &union_find[first_person as usize])];

        for &[x, y, time] in meetings {
            if time != prev_time {
                prev_time = time;

                Self::broadcast_in_time_frame(&union_find, &mut time_frame);
            }

            time_frame.push((&union_find[x as u32 as usize], &union_find[y as u32 as usize]));
        }

        Self::broadcast_in_time_frame(&union_find, &mut time_frame);

        let zero_root = Self::find_root(&union_find, &union_find[0]);

        union_find
            .iter()
            .filter(|&node| ptr::eq(Self::find_root(&union_find, node), zero_root))
            .map(|node| Self::get_node(&union_find, node) as _)
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        Self::find_all_people(n, meetings, first_person)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
