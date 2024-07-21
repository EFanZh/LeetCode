pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Edge {
    distance: u32,
    from: u16,
    to: u16,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl Solution {
    fn find_root(union_find: &mut [(u16, u16)], node: u16) -> u16 {
        let parent = union_find[usize::from(node)].0;

        if parent == u16::MAX {
            node
        } else {
            let root = Self::find_root(union_find, parent);

            union_find[usize::from(node)].0 = root;

            root
        }
    }

    fn union(union_find: &mut [(u16, u16)], left: u16, right: u16) -> bool {
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

    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut iter = (0..).zip(
            points
                .iter()
                .map(|point| <[_; 2]>::try_from(point.as_slice()).ok().unwrap()),
        );

        let mut edges = Vec::with_capacity(n * (n - 1) / 2);

        while let Some((from, [x_i, y_i])) = iter.next() {
            for (to, [x_j, y_j]) in iter.clone() {
                edges.push(Edge {
                    distance: ((x_j - x_i).abs() + (y_j - y_i).abs()) as _,
                    from,
                    to,
                });
            }
        }

        let mut queue = BinaryHeap::from(edges);
        let mut union_find = vec![(u16::MAX, 0); n].into_boxed_slice();
        let mut cost = 0;

        for _ in 1..n {
            loop {
                let edge = queue.pop().unwrap();

                if Self::union(&mut union_find, edge.from, edge.to) {
                    cost += edge.distance;

                    break;
                }
            }
        }

        cost as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        Self::min_cost_connect_points(points)
    }
}

#[cfg(test)]
mod tests {
    use super::Edge;

    #[test]
    fn test_edge_partial_eq() {
        assert!(
            Edge {
                distance: 2,
                from: 3,
                to: 5,
            } == Edge {
                distance: 2,
                from: 7,
                to: 9,
            },
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
