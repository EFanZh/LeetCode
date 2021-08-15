pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::mem;

trait Node {
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn parent(&self) -> Self;
    fn is_left_child(&self) -> bool;
    fn depth(&self) -> u32;
}

impl Node for usize {
    fn left(&self) -> Self {
        self * 2
    }

    fn right(&self) -> Self {
        self * 2 + 1
    }

    fn parent(&self) -> Self {
        self / 2
    }

    fn is_left_child(&self) -> bool {
        self % 2 == 0
    }

    fn depth(&self) -> u32 {
        8 * mem::size_of_val(self) as u32 - 1 - self.leading_zeros()
    }
}

struct SegmentTree {
    tree: Vec<i32>,
    lazy: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n * 2 - 1],
            lazy: vec![0; n - 1],
            n,
        }
    }

    fn update(&mut self, start: usize, end: usize, height: i32) {
        let (initial_left, initial_right) = (start + self.n, end + self.n);
        let (mut left, mut right) = (initial_left, initial_right);

        while left < right {
            if !left.is_left_child() {
                if let Some(lazy) = self.lazy.get_mut(left - 1) {
                    *lazy = height;
                }

                self.tree[left - 1] = height;

                left += 1;
            }

            if !right.is_left_child() {
                if let Some(lazy) = self.lazy.get_mut(right - 2) {
                    *lazy = height;
                }

                self.tree[right - 2] = height;

                right -= 1;
            }

            left = left.parent();
            right = right.parent();
        }

        for &node in &[initial_left, initial_right - 1] {
            let mut node = node / 2;

            loop {
                let value = &mut self.tree[node - 1];

                (*value) = (*value).max(height);

                if node == 1 {
                    break;
                }

                node = node.parent();
            }
        }
    }

    fn query(&mut self, start: usize, end: usize) -> i32 {
        let (mut left, mut right) = (start + self.n, end + self.n);

        for &node in &[left, right - 1] {
            for i in (1..=node.depth()).rev() {
                let ancestor = node >> i;
                let lazy = self.lazy[ancestor - 1];

                if lazy != 0 {
                    for &child in &[ancestor.left(), ancestor.right()] {
                        if let Some(child_lazy) = self.lazy.get_mut(child - 1) {
                            *child_lazy = lazy;
                        }

                        self.tree[child - 1] = lazy;
                    }

                    self.lazy[ancestor - 1] = 0;
                }
            }
        }

        let mut result = 0;

        while left < right {
            if !left.is_left_child() {
                result = result.max(self.tree[left - 1]);
                left += 1;
            }

            if !right.is_left_child() {
                result = result.max(self.tree[right - 2]);
                right -= 1;
            }

            left = left.parent();
            right = right.parent();
        }

        result
    }
}

impl Solution {
    fn inner(positions: &[(i32, i32)]) -> Vec<i32> {
        let indices = {
            let mut x_values = HashSet::with_capacity(positions.len() * 2);

            for &(left, length) in positions {
                x_values.insert(left);
                x_values.insert(left + length);
            }

            let mut x_values = x_values.into_iter().collect::<Vec<_>>();

            x_values.sort_unstable();

            x_values
                .into_iter()
                .enumerate()
                .map(|(i, x)| (x, i))
                .collect::<HashMap<_, _>>()
        };

        let mut result = Vec::with_capacity(positions.len());
        let n = indices.len() - 1;
        let mut segment_tree = SegmentTree::new(n);
        let mut tallest = 0;

        for &(left, length) in positions {
            let start = indices[&left];
            let end = indices[&(left + length)];
            let base = segment_tree.query(start, end);
            let height = base + length;

            segment_tree.update(start, end, height);

            tallest = tallest.max(height);

            result.push(tallest);
        }

        result
    }

    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        Self::inner(
            &positions
                .into_iter()
                .map(|position| {
                    let [left, length]: [i32; 2] = position.as_slice().try_into().unwrap();

                    (left, length)
                })
                .collect::<Vec<_>>(),
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        Self::falling_squares(positions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
