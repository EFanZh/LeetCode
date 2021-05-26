use std::mem;

struct NumArray {
    tree: Box<[i32]>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        fn build_tree(nums: &[i32], tree: &mut Vec<i32>) -> i32 {
            let (left, right) = nums.split_at(nums.len() / 2);

            let left_sum = if let [num] = *left {
                tree.push(num);
                num
            } else {
                build_tree(left, tree)
            };

            let right_sum = if let [num] = *right {
                tree.push(num);
                num
            } else {
                build_tree(right, tree)
            };

            let sum = left_sum + right_sum;

            tree.push(sum);

            sum
        }

        Self {
            tree: match nums.as_slice() {
                [] => Box::default(),
                &[num] => Box::new([num]),
                nums => {
                    let mut tree = Vec::with_capacity(nums.len() * 2 - 1);

                    build_tree(nums, &mut tree);

                    tree.into_boxed_slice()
                }
            },
        }
    }

    fn update(&mut self, i: i32, val: i32) {
        fn helper(tree: &mut [i32], i: usize, val: i32) -> i32 {
            if let [root] = tree {
                val - mem::replace(root, val)
            } else {
                let half = (tree.len() + 1) / 4;
                let (root, children) = tree.split_last_mut().unwrap();
                let (left, right) = children.split_at_mut(half * 2 - 1);

                let diff = if i < half {
                    helper(left, i, val)
                } else {
                    helper(right, i - half, val)
                };

                *root += diff;

                diff
            }
        }

        helper(&mut self.tree, i as _, val);
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        fn range_to(mut tree: &[i32], mut to: usize) -> i32 {
            let mut result = 0;

            while to != 0 {
                let length = (tree.len() + 1) / 2;
                let (root, children) = tree.split_last().unwrap();

                if to == length {
                    result += root;

                    break;
                }

                let half = length / 2;
                let (left, right) = children.split_at(half * 2 - 1);

                if to < half {
                    tree = left;
                } else {
                    result += *left.last().unwrap();
                    tree = right;
                    to -= half;
                }
            }

            result
        }

        range_to(&self.tree, j as usize + 1) - range_to(&self.tree, i as _)
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
