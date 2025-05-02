pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn fenwick_tree_add(tree: &mut [i32], mut value: usize) {
        while let Some(count) = tree.get_mut(value) {
            *count += 1;
            value |= value + 1;
        }
    }

    fn fenwick_tree_count_less_than(tree: &[i32], mut value: usize) -> i32 {
        let mut result = 0;

        loop {
            let value_minus_1 = value.wrapping_sub(1);

            if let Some(&count) = tree.get(value_minus_1) {
                result += count;
                value &= value_minus_1;
            } else {
                break;
            }
        }

        result
    }

    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let mut nums1 = nums1;

        nums1.iter_mut().zip(&nums2).for_each(|(lhs, rhs)| *lhs -= *rhs);

        let min = nums1.iter().fold(i32::MAX, |min, &num| min.min(num));

        for num in &mut nums1 {
            *num -= min;
        }

        let max = nums1.iter().fold(0, |max, &num| max.max(num as _));
        let mut fenwick_tree = nums2;

        fenwick_tree.clear();
        fenwick_tree.resize((max + 1) as _, 0);

        let mut result = 0_u64;

        (0..).zip(nums1).for_each(|(i, num)| {
            let target = num + diff;

            if target >= 0 {
                let target = target as u32;

                let count = if target < max {
                    u64::from(Self::fenwick_tree_count_less_than(&fenwick_tree, (target + 1) as _) as u32)
                } else {
                    i
                };

                result += count;
            }

            Self::fenwick_tree_add(&mut fenwick_tree, num as _);
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        Self::number_of_pairs(nums1, nums2, diff)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
