pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::cmp::Ordering;
use std::{mem, ptr};

type State = (Cell<u16>, Cell<u16>);

impl Solution {
    fn get_node(union_find: &[State], state: &State) -> u16 {
        ((ptr::from_ref(state) as usize - union_find.as_ptr() as usize) / mem::size_of::<State>()) as _
    }

    fn find_root<'a>(union_find: &'a [State], state: &'a State) -> &'a State {
        let parent = state.0.get();

        union_find.get(parent as usize).map_or(state, |parent_state| {
            let root = Self::find_root(union_find, parent_state);

            state.0.set(Self::get_node(union_find, root));

            root
        })
    }

    fn union<'a>(union_find: &'a [State], left_root: &'a State, right_root: &'a State) -> (&'a State, &'a State) {
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

        child.0.set(Self::get_node(union_find, parent));

        (child, parent)
    }

    fn ptr_cmp(lhs: *const State, rhs: *const State) -> Ordering {
        lhs.cmp(&rhs)
    }

    fn merge<'a>(left: &[&'a State], right: &[&'a State], buffer: &mut Vec<&'a State>) {
        let mut left_iter = left.iter().copied();
        let mut right_iter = right.iter().copied();

        'outer: while let Some(mut left) = left_iter.next() {
            for right in right_iter.by_ref() {
                loop {
                    match Self::ptr_cmp(left, right) {
                        Ordering::Less => {
                            buffer.push(left);

                            if let Some(next_left) = left_iter.next() {
                                left = next_left;
                            } else {
                                buffer.push(right);

                                break 'outer;
                            }
                        }
                        Ordering::Equal => {
                            buffer.push(left);

                            continue 'outer;
                        }
                        Ordering::Greater => {
                            buffer.push(right);

                            break;
                        }
                    }
                }
            }

            buffer.push(left);
            right_iter = left_iter;

            break;
        }

        buffer.extend(right_iter);
    }

    pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as u32 as usize;
        let union_find = vec![(Cell::new(u16::MAX), Cell::new(0)); n].into_boxed_slice();
        let mut root_restrictions = vec![Vec::new(); n].into_boxed_slice();
        let parse = |value: Vec<_>| <[_; 2]>::map(value.try_into().ok().unwrap(), |x| x as u32 as usize);

        for restriction in restrictions {
            let [left, right] = parse(restriction);
            let left_node = &union_find[left];
            let right_node = &union_find[right];

            for (node, other) in [(left, right_node), (right, left_node)] {
                root_restrictions[node].push(other);
            }
        }

        root_restrictions.iter_mut().for_each(|restrictions| {
            restrictions.sort_unstable_by_key(|&node| ptr::from_ref(node));
            restrictions.dedup_by_key(|&mut node| ptr::from_ref(node));
        });

        let mut buffer = Vec::new();

        requests
            .into_iter()
            .map(|request| {
                let [left, right] = parse(request);
                let left_node = &union_find[left];
                let right_node = &union_find[right];
                let left_root = Self::find_root(&union_find, left_node);
                let right_root = Self::find_root(&union_find, right_node);

                ptr::eq(left_root, right_root) || {
                    let success = root_restrictions[usize::from(Self::get_node(&union_find, left_root))]
                        .iter()
                        .all(|restriction| !ptr::eq(Self::find_root(&union_find, restriction), right_root));

                    if success {
                        let (child, parent) = Self::union(&union_find, left_root, right_root);
                        let child_index = usize::from(Self::get_node(&union_find, child));
                        let parent_index = usize::from(Self::get_node(&union_find, parent));

                        Self::merge(
                            &root_restrictions[child_index],
                            &root_restrictions[parent_index],
                            &mut buffer,
                        );

                        mem::swap(&mut root_restrictions[parent_index], &mut buffer);

                        let child_restrictions = &mut root_restrictions[child_index];

                        if child_restrictions.capacity() > buffer.capacity() {
                            mem::swap(child_restrictions, &mut buffer);
                        }

                        buffer.clear();
                    }

                    success
                }
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        Self::friend_requests(n, restrictions, requests)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
