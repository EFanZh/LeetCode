pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn find_root(union_find: &mut [(u16, u16)], node: u16) -> (u16, u16) {
        let state = union_find[usize::from(node)];

        if state.0 == 0 {
            (node, state.1)
        } else {
            let result = Self::find_root(union_find, state.0);

            if state.0 != result.0 {
                union_find[usize::from(node)].0 = result.0;
            }

            result
        }
    }

    fn union(union_find: &mut [(u16, u16)], left: u16, right: u16) {
        let left_root = Self::find_root(union_find, left);
        let right_root = Self::find_root(union_find, right);

        if left_root.0 != right_root.0 {
            let (child, parent) = if left_root.1 < right_root.1 {
                (left_root.0, right_root.0)
            } else {
                if left_root.1 == right_root.1 {
                    union_find[usize::from(left_root.0)].1 += 1;
                }

                (right_root.0, left_root.0)
            };

            union_find[usize::from(child)].0 = parent;
        }
    }

    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as u16;
        let threshold = threshold as u16 + 1;
        let mut union_find = vec![(0, 0); usize::from(n) + 1];

        assert!(threshold != 0);

        for left in threshold..=n / 2 {
            for right in (left * 2..=n).step_by(usize::from(left)) {
                Self::union(&mut union_find, left, right);
            }
        }

        queries
            .into_iter()
            .map(|query| {
                let [a, b]: [_; 2] = query.try_into().ok().unwrap();

                Self::find_root(&mut union_find, a as _) == Self::find_root(&mut union_find, b as _)
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::are_connected(n, threshold, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
