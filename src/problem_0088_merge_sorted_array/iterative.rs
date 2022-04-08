pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge_helper(nums1: &mut [i32], mut m: usize, mut left: i32, mut nums2: &[i32], mut right: i32) {
        loop {
            let i = m + nums2.len() + 1;

            if right < left {
                nums1[i] = left;

                m = m.wrapping_sub(1);

                if let Some(&new_left) = nums1.get(m) {
                    left = new_left;
                } else {
                    nums1[..nums2.len()].copy_from_slice(nums2);
                    nums1[nums2.len()] = right;

                    break;
                }
            } else {
                nums1[i] = right;

                if let Some((&new_right, rest_right)) = nums2.split_last() {
                    nums2 = rest_right;
                    right = new_right;
                } else {
                    break;
                }
            }
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let n = n as usize;
        let nums1 = &mut nums1[..m + n];
        let nums2 = &nums2[..n];

        if let Some((&right, rest_right)) = nums2.split_last() {
            m = m.wrapping_sub(1);

            if let Some(&left) = nums1.get(m) {
                Self::merge_helper(nums1, m, left, rest_right, right);
            } else {
                nums1[..nums2.len()].copy_from_slice(nums2);
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        Self::merge(nums1, m, nums2, n);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
