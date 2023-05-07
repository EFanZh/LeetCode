pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryInto;

const UNKNOWN: u8 = 0;
const MAYBE_CRITICAL: u8 = 1;
const PSEUDO_CRITICAL: u8 = 2;

struct Edge {
    weight: u16,
    index: u8,
    from: u8,
    to: u8,
}

struct Context<'a> {
    forest: &'a [Vec<(u16, u8, u8)>],
    states: &'a mut [u8],
    weight: u16,
    target: u8,
}

impl Solution {
    fn find_root(union_find: &mut [(u8, u8)], node: u8) -> u8 {
        let mut candidate = union_find[usize::from(node)].0;

        if candidate == u8::MAX {
            node
        } else {
            candidate = Self::find_root(union_find, candidate);

            union_find[usize::from(node)].0 = candidate;

            candidate
        }
    }

    fn union(union_find: &mut [(u8, u8)], left: u8, right: u8) -> bool {
        let left_root = Self::find_root(union_find, left);
        let right_root = Self::find_root(union_find, right);

        if left_root == right_root {
            false
        } else {
            match union_find[usize::from(left_root)]
                .1
                .cmp(&union_find[usize::from(right_root)].1)
            {
                Ordering::Less => union_find[usize::from(left_root)].0 = right_root,
                Ordering::Equal => {
                    union_find[usize::from(left_root)].1 += 1;
                    union_find[usize::from(right_root)].0 = left_root;
                }
                Ordering::Greater => union_find[usize::from(right_root)].0 = left_root,
            }

            true
        }
    }

    fn dfs(context: &mut Context, parent: u8, node: u8) -> Option<bool> {
        for &(weight, index, neighbor) in &context.forest[usize::from(node)] {
            if neighbor != parent {
                let fallback_has_same_weight = if neighbor == context.target {
                    false
                } else if let Some(has_same_weight) = Self::dfs(context, node, neighbor) {
                    has_same_weight
                } else {
                    continue;
                };

                return Some(if weight == context.weight {
                    context.states[usize::from(index)] = PSEUDO_CRITICAL;

                    true
                } else {
                    fallback_has_same_weight
                });
            }
        }

        None
    }

    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as u8;

        let mut edges = (0..)
            .zip(edges)
            .map(|(index, edge)| {
                let [from, to, weight]: [_; 3] = edge.as_slice().try_into().ok().unwrap();

                Edge {
                    weight: weight as _,
                    index,
                    from: from as _,
                    to: to as _,
                }
            })
            .collect::<Box<_>>();

        edges.sort_unstable_by_key(|edge| edge.weight);

        let mut union_find = vec![(u8::MAX, 0); usize::from(n)].into_boxed_slice();
        let mut forest = vec![Vec::new(); usize::from(n)].into_boxed_slice();
        let mut states = vec![UNKNOWN; edges.len()].into_boxed_slice();

        for edge in &*edges {
            states[usize::from(edge.index)] = if Self::union(&mut union_find, edge.from, edge.to) {
                forest[usize::from(edge.from)].push((edge.weight, edge.index, edge.to));
                forest[usize::from(edge.to)].push((edge.weight, edge.index, edge.from));

                MAYBE_CRITICAL
            } else if Self::dfs(
                &mut Context {
                    forest: &forest,
                    states: &mut states,
                    weight: edge.weight,
                    target: edge.to,
                },
                u8::MAX,
                edge.from,
            ) == Some(true)
            {
                PSEUDO_CRITICAL
            } else {
                continue;
            };
        }

        let mut critical_edges = Vec::new();
        let mut pseudo_critical_edge = Vec::new();

        for (i, &state) in (0..).zip(&*states) {
            match state {
                MAYBE_CRITICAL => critical_edges.push(i),
                PSEUDO_CRITICAL => pseudo_critical_edge.push(i),
                _ => {}
            }
        }

        vec![critical_edges, pseudo_critical_edge]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::find_critical_and_pseudo_critical_edges(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
