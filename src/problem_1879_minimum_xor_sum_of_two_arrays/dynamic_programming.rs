pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        assert_eq!(nums1.len(), nums2.len());

        let n = nums1.len();
        let mut cache = vec![0_u32; 1 << n].into_boxed_slice();

        for selection in 1_usize..cache.len() {
            let bottom_num = nums2[(selection.count_ones() - 1) as usize];
            let mut value = u32::MAX;
            let mut remaining_bits = selection;

            loop {
                let top_index = remaining_bits.trailing_zeros();
                let probe = 1 << top_index;

                value = value.min(cache[selection ^ probe] + (nums1[top_index as usize] ^ bottom_num) as u32);

                remaining_bits ^= probe;

                if remaining_bits == 0 {
                    break;
                }
            }

            cache[selection] = value;
        }

        cache.last().copied().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::minimum_xor_sum(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
