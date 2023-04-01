pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let nums1 = nums1.as_slice();
        let nums2 = nums2.as_slice();
        let m = nums1.len();
        let n = nums2.len();

        assert!(m > 0 && m <= 500 && n > 0 && n <= 500);

        let mut cache = vec![0; m * n].into_boxed_slice();

        let mut iter = cache.chunks_exact_mut(n).zip(nums1);
        let (cache_row, &lhs) = iter.next().unwrap();

        // First row.

        let mut left = i32::MIN;

        for (target, rhs) in cache_row.iter_mut().zip(nums2) {
            left = left.max(lhs * rhs);
            *target = left;
        }

        // Rest rows.

        let mut prev_row = &*cache_row;

        for (cache_row, &lhs) in iter {
            let mut top_left = i32::MIN;

            left = i32::MIN;

            for ((target, rhs), &top) in cache_row.iter_mut().zip(nums2).zip(prev_row) {
                left = left.max(top).max(top_left.max(0) + lhs * rhs);
                top_left = top;
                *target = left;
            }

            prev_row = cache_row;
        }

        left
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::max_dot_product(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
