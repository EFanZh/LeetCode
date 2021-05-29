pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    fn prefix_sum(tree: &[usize], mut length: usize) -> usize {
        let mut result = 0;

        while let Some(value) = tree.get(length.wrapping_sub(1)) {
            result += value;
            length &= length - 1;
        }

        result
    }

    fn update(tree: &mut [usize], mut index: usize) {
        tree[index] += 1;
        index |= index + 1;

        while let Some(value) = tree.get_mut(index) {
            *value += 1;
            index |= index + 1;
        }
    }

    fn find_nth_free_slot(tree: &[usize], n: usize) -> usize {
        let mut left = 0;
        let mut right = tree.len() - 1;

        while left != right {
            let middle = left + (right - left) / 2;

            if middle < n + Self::prefix_sum(tree, middle + 1) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left
    }

    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by_key(|p| {
            let [h, k]: [_; 2] = p.as_slice().try_into().unwrap();

            (h, Reverse(k))
        });

        let mut result = vec![Vec::new(); people.len()];
        let mut fenwick_tree = vec![0; people.len()];

        for p in people {
            let index = Self::find_nth_free_slot(&fenwick_tree, p[1] as _);

            result[index] = p;
            Self::update(&mut fenwick_tree, index);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::reconstruct_queue(people)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
