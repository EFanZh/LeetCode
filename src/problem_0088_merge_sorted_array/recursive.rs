pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn merge_helper_left(nums1: &mut [i32], length1: usize, left: i32, nums2: &[i32]) {
        if let Some((&right, rest_right)) = nums2.split_last() {
            Self::merge_helper_both(nums1, length1, left, rest_right, right);
        }
    }

    fn merge_helper_right(nums1: &mut [i32], mut length1: usize, nums2: &[i32], right: i32) {
        length1 = length1.wrapping_sub(1);

        if let Some(&left) = nums1.get(length1) {
            Self::merge_helper_both(nums1, length1, left, nums2, right);
        } else {
            nums1[..nums2.len()].copy_from_slice(nums2);
            nums1[nums2.len()] = right;
        }
    }

    fn merge_helper_both(nums1: &mut [i32], length1: usize, left: i32, nums2: &[i32], right: i32) {
        let i = length1 + nums2.len() + 1;

        if right < left {
            nums1[i] = left;

            Self::merge_helper_right(nums1, length1, nums2, right);
        } else {
            nums1[i] = right;

            Self::merge_helper_left(nums1, length1, left, nums2);
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let nums1 = &mut nums1[..m + n];
        let nums2 = &nums2[..n];

        if let Some((&right, rest_right)) = nums2.split_last() {
            Self::merge_helper_right(nums1, m, rest_right, right);
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
