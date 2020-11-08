struct NumArray {
    tree: Box<[i32]>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            tree: (0..nums.len()).map(|i| nums[i & (i + 1)..=i].iter().sum()).collect(),
        }
    }

    fn prefix_sum(tree: &[i32], mut i: usize) -> i32 {
        let mut result = 0;

        while i != 0 {
            result += tree[i - 1];

            i &= i - 1;
        }

        result
    }

    fn update(&mut self, i: i32, val: i32) {
        let tree = self.tree.as_mut();
        let mut i = i as usize;
        let old_val = Self::prefix_sum(tree, i + 1) - Self::prefix_sum(tree, i);
        let diff = val - old_val;

        loop {
            tree[i] += diff;

            i += !i & (i + 1);

            if i >= tree.len() {
                break;
            }
        }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let tree = self.tree.as_ref();

        Self::prefix_sum(tree, j as usize + 1) - Self::prefix_sum(tree, i as _)
    }
}

impl super::NumArray for NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self::new(nums)
    }

    fn update(&mut self, i: i32, val: i32) {
        self.update(i, val)
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
