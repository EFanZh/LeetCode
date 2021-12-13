pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://en.wikipedia.org/wiki/Klee%27s_measure_problem>.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

struct Node {
    count: u32,
    covered: u32,
}

struct Tree<'a> {
    root: &'a mut Node,
    left: &'a mut [Node],
    right: &'a mut [Node],
    start: u32,
    length: u32,
}

impl<'a> Tree<'a> {
    fn from_slice(slice: &'a mut [Node], start: u32) -> Self {
        let length = (slice.len() + 1) / 2;
        let (root, children) = slice.split_first_mut().unwrap();
        let left_length = length / 2;
        let (left, right) = children.split_at_mut((left_length * 2).saturating_sub(1));

        Self {
            root,
            left,
            right,
            start,
            length: length as _,
        }
    }

    fn as_non_leaf_tree(&mut self) -> NonLeafTree {
        let left_length = self.length / 2;

        NonLeafTree {
            root: self.root,
            left: Tree::from_slice(self.left, self.start),
            right: Tree::from_slice(self.right, self.start + left_length),
            start: self.start,
            length: self.length,
        }
    }

    fn end(&self) -> u32 {
        self.start + self.length
    }
}

struct NonLeafTree<'a> {
    root: &'a mut Node,
    left: Tree<'a>,
    right: Tree<'a>,
    start: u32,
    length: u32,
}

impl NonLeafTree<'_> {
    fn middle(&self) -> u32 {
        self.start + self.length / 2
    }
}

impl Solution {
    fn segment_tree_insert(tree: &mut Tree, ys: &[u32], start: u32, end: u32) {
        fn insert_left_aligned(mut tree: NonLeafTree, ys: &[u32], start: u32, end: u32) {
            let middle = tree.middle();

            if end < middle {
                insert_left_aligned(tree.left.as_non_leaf_tree(), ys, start, end);
            } else {
                insert_aligned(tree.left.root, ys, start, middle);

                if end > middle {
                    insert_left_aligned(tree.right.as_non_leaf_tree(), ys, middle, end);
                }
            }

            if tree.root.count == 0 {
                tree.root.covered = tree.left.root.covered + tree.right.root.covered;
            }
        }

        fn insert_right_aligned(mut tree: NonLeafTree, ys: &[u32], start: u32, end: u32) {
            let middle = tree.middle();

            if start > middle {
                insert_right_aligned(tree.right.as_non_leaf_tree(), ys, start, end);
            } else {
                insert_aligned(tree.right.root, ys, middle, end);

                if start < middle {
                    insert_right_aligned(tree.left.as_non_leaf_tree(), ys, start, middle);
                }
            }

            if tree.root.count == 0 {
                tree.root.covered = tree.left.root.covered + tree.right.root.covered;
            }
        }

        fn insert_not_aligned(mut tree: NonLeafTree, ys: &[u32], start: u32, end: u32) {
            let middle = tree.middle();

            match start.cmp(&middle) {
                Ordering::Less => match end.cmp(&middle) {
                    Ordering::Less => insert_not_aligned(tree.left.as_non_leaf_tree(), ys, start, end),
                    Ordering::Equal => insert_right_aligned(tree.left.as_non_leaf_tree(), ys, start, end),
                    Ordering::Greater => {
                        insert_right_aligned(tree.left.as_non_leaf_tree(), ys, start, middle);
                        insert_left_aligned(tree.right.as_non_leaf_tree(), ys, middle, end);
                    }
                },
                Ordering::Equal => insert_left_aligned(tree.right.as_non_leaf_tree(), ys, start, end),
                Ordering::Greater => insert_not_aligned(tree.right.as_non_leaf_tree(), ys, start, end),
            }

            if tree.root.count == 0 {
                tree.root.covered = tree.left.root.covered + tree.right.root.covered;
            }
        }

        fn insert_aligned(root: &mut Node, ys: &[u32], start: u32, end: u32) {
            if root.count == 0 {
                root.covered = ys[end as usize] - ys[start as usize];
            }

            root.count += 1;
        }

        if start == tree.start {
            if end == tree.end() {
                insert_aligned(tree.root, ys, start, end);
            } else {
                insert_left_aligned(tree.as_non_leaf_tree(), ys, start, end);
            }
        } else if end == tree.end() {
            insert_right_aligned(tree.as_non_leaf_tree(), ys, start, end);
        } else {
            insert_not_aligned(tree.as_non_leaf_tree(), ys, start, end);
        }
    }

    fn segment_tree_remove(tree: &mut Tree, ys: &[u32], start: u32, end: u32) {
        fn remove_left_aligned(mut tree: NonLeafTree, ys: &[u32], start: u32, end: u32) {
            let middle = tree.middle();

            if end < middle {
                remove_left_aligned(tree.left.as_non_leaf_tree(), ys, start, end);
            } else {
                remove_aligned(&mut tree.left);

                if end > middle {
                    remove_left_aligned(tree.right.as_non_leaf_tree(), ys, middle, end);
                }
            }

            if tree.root.count == 0 {
                tree.root.covered = tree.left.root.covered + tree.right.root.covered;
            }
        }

        fn remove_right_aligned(mut tree: NonLeafTree, ys: &[u32], start: u32, end: u32) {
            let middle = tree.middle();

            if start > middle {
                remove_right_aligned(tree.right.as_non_leaf_tree(), ys, start, end);
            } else {
                remove_aligned(&mut tree.right);

                if start < middle {
                    remove_right_aligned(tree.left.as_non_leaf_tree(), ys, start, middle);
                }
            }

            if tree.root.count == 0 {
                tree.root.covered = tree.left.root.covered + tree.right.root.covered;
            }
        }

        fn remove_not_aligned(mut tree: NonLeafTree, ys: &[u32], start: u32, end: u32) {
            let middle = tree.middle();

            match start.cmp(&middle) {
                Ordering::Less => match end.cmp(&middle) {
                    Ordering::Less => remove_not_aligned(tree.left.as_non_leaf_tree(), ys, start, end),
                    Ordering::Equal => remove_right_aligned(tree.left.as_non_leaf_tree(), ys, start, end),
                    Ordering::Greater => {
                        remove_right_aligned(tree.left.as_non_leaf_tree(), ys, start, middle);
                        remove_left_aligned(tree.right.as_non_leaf_tree(), ys, middle, end);
                    }
                },
                Ordering::Equal => remove_left_aligned(tree.right.as_non_leaf_tree(), ys, start, end),
                Ordering::Greater => remove_not_aligned(tree.right.as_non_leaf_tree(), ys, start, end),
            }

            if tree.root.count == 0 {
                tree.root.covered = tree.left.root.covered + tree.right.root.covered;
            }
        }

        fn remove_aligned(mut tree: &mut Tree) {
            tree.root.count -= 1;

            if tree.root.count == 0 {
                tree.root.covered = if let (Some(left), Some(right)) = (tree.left.first(), tree.right.first()) {
                    left.covered + right.covered
                } else {
                    0
                };
            }
        }

        if start == tree.start {
            if end == tree.end() {
                remove_aligned(tree);
            } else {
                remove_left_aligned(tree.as_non_leaf_tree(), ys, start, end);
            }
        } else if end == tree.end() {
            remove_right_aligned(tree.as_non_leaf_tree(), ys, start, end);
        } else {
            remove_not_aligned(tree.as_non_leaf_tree(), ys, start, end);
        }
    }

    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        // Build events and compress y coordinates.

        let mut events = Vec::with_capacity(rectangles.len() * 2);
        let mut ys = HashSet::new();

        for rectangle in &rectangles {
            let [x_1, y_1, x_2, y_2]: [_; 4] = rectangle.as_slice().try_into().unwrap();
            let (x_1, y_1, x_2, y_2) = (x_1 as u32, y_1 as u32, x_2 as u32, y_2 as u32);

            ys.extend(&[y_1, y_2]);

            events.push((x_1, y_1, y_2));
            events.push((x_2, y_1 | 0x8000_0000, y_2));
        }

        let mut ys = ys.into_iter().collect::<Vec<_>>();

        ys.sort_unstable();

        let y_map = ys.iter().copied().zip(0..).collect::<HashMap<_, _>>();

        events.sort_unstable_by_key(|&(x, _, _)| x);

        // Build segment tree.

        let mut nodes = (0..ys.len() * 2 - 3)
            .map(|_| Node { count: 0, covered: 0 })
            .collect::<Vec<_>>();

        let mut tree = Tree::from_slice(&mut nodes, 0);

        // Run events and calculate result.

        let mut result = 0;
        let mut prev_x = 0;

        for &(x, y_1, y_2) in &events {
            if x != prev_x {
                result += u64::from(tree.root.covered) * u64::from(x - prev_x);

                prev_x = x;
            }

            if y_1 & 0x8000_0000 == 0 {
                Self::segment_tree_insert(&mut tree, &ys, y_map[&y_1], y_map[&y_2]);
            } else {
                Self::segment_tree_remove(&mut tree, &ys, y_map[&(y_1 & 0x7FFF_FFFF)], y_map[&y_2]);
            }
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        Self::rectangle_area(rectangles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
