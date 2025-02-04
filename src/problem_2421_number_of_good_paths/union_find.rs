pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::cmp::Ordering;
use std::{mem, ptr};

struct Node {
    parent: Cell<u32>,
    rank: Cell<u32>,
    max_value: Cell<u32>,
    max_value_count: Cell<u32>,
}

impl Solution {
    fn get_node(union_find: &[Node], node: &Node) -> usize {
        (ptr::from_ref(node) as usize - union_find.as_ptr() as usize) / mem::size_of::<Node>()
    }

    fn find_root<'a>(union_find: &'a [Node], node: &'a Node) -> &'a Node {
        let parent = node.parent.get();

        union_find.get(parent as usize).map_or(node, |parent_node| {
            let root = Self::find_root(union_find, parent_node);

            node.parent.set(Self::get_node(union_find, root) as _);

            root
        })
    }

    fn union(union_find: &[Node], left_node: &Node, right_node: &Node, result: &mut u32) {
        let left_root = Self::find_root(union_find, left_node);
        let right_root = Self::find_root(union_find, right_node);
        let left_rank = left_root.rank.get();
        let right_rank = right_root.rank.get();

        let (child, parent) = if left_rank < right_rank {
            (left_root, right_root)
        } else {
            if left_rank == right_rank {
                left_root.rank.set(left_rank + 1);
            }

            (right_root, left_root)
        };

        match child.max_value.cmp(&parent.max_value) {
            Ordering::Less => {}
            Ordering::Equal => {
                let child_max_value_count = child.max_value_count.get();
                let parent_max_value_count = parent.max_value_count.get();

                *result += child_max_value_count * parent_max_value_count;

                parent
                    .max_value_count
                    .set(parent_max_value_count + child_max_value_count);
            }
            Ordering::Greater => {
                parent.max_value.set(child.max_value.get());
                parent.max_value_count.set(child.max_value_count.get());
            }
        }

        child.parent.set(Self::get_node(union_find, parent) as _);
    }

    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let as_u32 = |x| x as u32;

        let mut edges = edges
            .into_iter()
            .map(|edge| {
                let [x, y] = <[_; 2]>::map(edge.try_into().ok().unwrap(), as_u32);

                (u32::max(vals[x as usize] as _, vals[y as usize] as _), x, y)
            })
            .collect::<Box<_>>();

        edges.sort_unstable_by_key(|&(val, ..)| val);

        let union_find = vals
            .into_iter()
            .map(|val| Node {
                parent: Cell::new(u32::MAX),
                rank: Cell::new(0),
                max_value: Cell::new(val as _),
                max_value_count: Cell::new(1),
            })
            .collect::<Box<_>>();

        let mut result = union_find.len() as u32;

        for &(_, x, y) in &*edges {
            Self::union(
                &union_find,
                &union_find[x as usize],
                &union_find[y as usize],
                &mut result,
            );
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        Self::number_of_good_paths(vals, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
