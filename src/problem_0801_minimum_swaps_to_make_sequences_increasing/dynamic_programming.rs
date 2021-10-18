pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min_swap_1 = 0_u32;
        let mut min_swap_2 = 0_u32;
        let mut prev_left = i32::MIN;
        let mut prev_right = i32::MIN;

        for (&left, &right) in nums1.iter().zip(&nums2) {
            let (new_min_swap_1, new_min_swap_2) = if left > prev_left && right > prev_right {
                if left > prev_right && right > prev_left {
                    let new_min_swap_1 = min_swap_1.min(min_swap_2);

                    (new_min_swap_1, new_min_swap_1 + 1)
                } else {
                    (min_swap_1, min_swap_2 + 1)
                }
            } else {
                (min_swap_2, min_swap_1 + 1)
            };

            min_swap_1 = new_min_swap_1;
            min_swap_2 = new_min_swap_2;
            prev_left = left;
            prev_right = right;
        }

        min_swap_1.min(min_swap_2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::min_swap(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
