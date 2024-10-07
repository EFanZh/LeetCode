pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn fenwick_tree_sum_less_than(tree: &[u32], mut x: usize) -> u32 {
        let mut result = 0;

        loop {
            let x_minus_1 = x.wrapping_sub(1);

            if let Some(&count) = tree.get(x_minus_1) {
                result += count;

                x &= x_minus_1;
            } else {
                break;
            }
        }

        result
    }

    fn fenwick_tree_add(tree: &mut [u32], mut x: usize) {
        while let Some(count) = tree.get_mut(x) {
            *count += 1;

            x |= x + 1;
        }
    }

    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let mut indices = vec![0; n].into_boxed_slice();

        for (i, num) in (0_u32..).zip(nums1) {
            indices[num as u32 as usize] = i;
        }

        let mut fenwick_tree = vec![0; n].into_boxed_slice();
        let mut result = 0;
        let mut right_count = n as u32;

        for num in nums2 {
            right_count -= 1;

            let index = indices[num as u32 as usize];
            let left_less = Self::fenwick_tree_sum_less_than(&fenwick_tree, index as _);
            let right_greater = right_count - (index - left_less);

            result += u64::from(left_less) * u64::from(right_greater);
            Self::fenwick_tree_add(&mut fenwick_tree, index as _);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        Self::good_triplets(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
