// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

pub struct MajorityChecker {
    tree: Box<[(i32, u32)]>,
    indices: HashMap<i32, Vec<u32>>,
}

impl MajorityChecker {
    fn merge_state(target: &mut (i32, u32), new: (i32, u32)) {
        if new.0 == target.0 {
            target.1 += new.1;
        } else if target.1 < new.1 {
            target.0 = new.0;
            target.1 = new.1 - target.1;
        } else {
            target.1 -= new.1;
        }
    }

    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut tree = vec![(0, 0); n * 2 - 1].into_boxed_slice();

        for (target, &value) in tree[n - 1..].iter_mut().zip(&arr) {
            *target = (value, 1);
        }

        for i in (0..n - 1).rev() {
            let left_index = i * 2 + 1;
            let right_index = left_index + 1;
            let mut state = tree[left_index];

            Self::merge_state(&mut state, tree[right_index]);

            tree[i] = state;
        }

        let mut indices = HashMap::<_, Vec<_>>::new();

        for (i, value) in (0..).zip(arr) {
            indices
                .entry(value)
                .and_modify(|indices| indices.push(i))
                .or_insert_with(|| vec![i]);
        }

        Self { tree, indices }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let left = left as u32;
        let right = right as u32;
        let threshold = threshold as u32;
        let first_leaf = self.tree.len() / 2;
        let mut i = first_leaf + left as usize;
        let mut j = first_leaf + right as usize;
        let mut state = (0, 0);

        loop {
            if i.is_multiple_of(2) {
                Self::merge_state(&mut state, self.tree[i]);
            }

            if !j.is_multiple_of(2) {
                Self::merge_state(&mut state, self.tree[j]);
            }

            i /= 2;
            j /= 2;

            if i == j {
                break;
            }

            j -= 1;
        }

        if state.1 == 0 {
            -1
        } else {
            let indices = self.indices[&state.0].as_slice();
            let first_index = indices.partition_point(|&i| i < left);
            let count = indices[first_index..].partition_point(|&i| i <= right) as u32;

            if count < threshold { -1 } else { state.0 }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MajorityChecker for MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        Self::new(arr)
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        self.query(left, right, threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MajorityChecker>();
    }
}
