use std::iter;
use std::mem;

struct NumArray {
    tree: Box<[i32]>,
}

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        Self {
            tree: if nums.is_empty() {
                Box::new([])
            } else {
                let internal_nodes = nums.len() - 1;

                nums.splice(..0, iter::repeat(0).take(internal_nodes));

                let mut tree = nums.into_boxed_slice();

                for i in (0..internal_nodes).rev() {
                    tree[i] = tree[i * 2 + 1] + tree[(i + 1) * 2];
                }

                tree
            },
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        let tree = self.tree.as_mut();
        let mut i = tree.len() / 2 + i as usize;
        let diff = val - mem::replace(&mut tree[i], val);

        while i != 0 {
            i = (i - 1) / 2;
            tree[i] += diff;
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let tree = self.tree.as_ref();
        let internal_nodes = tree.len() / 2;
        let mut i = internal_nodes + i as usize;
        let mut j = internal_nodes + j as usize + 1;
        let mut result = 0;

        while i != j {
            if i % 2 == 0 {
                result += tree[i];
            }

            if j % 2 == 0 {
                result += tree[j - 1];
            }

            i /= 2;
            j = (j - 1) / 2;
        }

        result
    }
}

impl super::NumArray for NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self::new(nums)
    }

    fn update(&mut self, i: i32, val: i32) {
        self.update(i, val);
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.sum_range(i, j)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NumArray>();
    }
}
