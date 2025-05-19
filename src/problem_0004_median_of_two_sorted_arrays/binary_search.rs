pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums2.len() < nums1.len() {
            (nums2.as_slice(), nums1.as_slice())
        } else {
            (nums1.as_slice(), nums2.as_slice())
        };

        let mut left = 0;
        let mut right = nums1.len();
        let total = nums1.len() + nums2.len();
        let half = total / 2;
        let half_minus_1 = half - 1;

        while left < right {
            let middle = left + (right - left) / 2;

            if nums1[middle] < nums2[half_minus_1 - middle] {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        let index = left;
        let other = half - index;

        let right_min = f64::from(i32::min(
            nums1.get(index).copied().unwrap_or(i32::MAX),
            nums2.get(other).copied().unwrap_or(i32::MAX),
        ));

        if total % 2 == 0 {
            let left_max = f64::from(i32::max(
                nums1.get(index.wrapping_sub(1)).copied().unwrap_or(i32::MIN),
                nums2.get(other.wrapping_sub(1)).copied().unwrap_or(i32::MIN),
            ));

            f64::midpoint(left_max, right_min)
        } else {
            right_min
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Self::find_median_sorted_arrays(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
