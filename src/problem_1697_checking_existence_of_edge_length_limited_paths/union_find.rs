pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn find_root_helper(union_find: &mut [(u32, u32)], node: u32) -> Option<(u32, u32)> {
        let node = node as usize;

        union_find.get(node).copied().map(|(parent_plus_1, rank)| {
            let root = if let Some((root, _)) = Self::find_root_helper(union_find, parent_plus_1.wrapping_sub(1)) {
                union_find[node].0 = root + 1;

                root
            } else {
                node as _
            };

            (root, rank)
        })
    }

    fn find_root(union_find: &mut [(u32, u32)], node: u32) -> (u32, u32) {
        Self::find_root_helper(union_find, node).unwrap()
    }

    fn union(union_find: &mut [(u32, u32)], x: u32, y: u32) {
        let left = Self::find_root(union_find, x);
        let right = Self::find_root(union_find, y);

        if left.0 != right.0 {
            let (child, parent) = if left.1 < right.1 {
                (left.0, right.0)
            } else {
                if left.1 == right.1 {
                    union_find[left.0 as usize].1 += 1;
                }

                (right.0, left.0)
            };

            union_find[child as usize].0 = parent + 1;
        }
    }

    pub fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut edge_list = edge_list
            .into_iter()
            .map(|edge| {
                let [from, to, distance]: [_; 3] = edge.try_into().ok().unwrap();

                (distance as u32, from as u32, to as u32)
            })
            .collect::<Box<_>>();

        edge_list.sort_unstable_by_key(|&(distance, ..)| distance);

        let mut result = vec![false; queries.len()];

        let mut queue = result
            .iter_mut()
            .zip(queries)
            .map(|(slot, query)| {
                let [from, to, limit]: [_; 3] = query.try_into().ok().unwrap();

                (limit as u32, from as u32, to as u32, slot)
            })
            .collect::<Vec<_>>();

        queue.sort_unstable_by_key(|&(limit, ..)| limit);

        let mut iter = edge_list.iter();
        let mut union_find = vec![(0, 0); n as u32 as usize].into_boxed_slice();

        for (limit, p, q, slot) in queue {
            while let Some(item) = iter.as_slice().first() {
                if item.0 < limit {
                    Self::union(&mut union_find, item.1, item.2);

                    iter.next();
                } else {
                    break;
                }
            }

            *slot = Self::find_root(&mut union_find, p).0 == Self::find_root(&mut union_find, q).0;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::distance_limited_paths_exist(n, edge_list, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
