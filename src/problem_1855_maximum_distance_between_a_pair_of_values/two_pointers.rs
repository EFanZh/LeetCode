pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut j = 0;
        let mut result = 0;

        'outer: for (i, &left) in nums1.iter().enumerate() {
            loop {
                if let Some(&right) = nums2.get(j) {
                    if right < left {
                        break;
                    }

                    j += 1;
                } else {
                    result = result.max(j.saturating_sub(i + 1));

                    break 'outer;
                }
            }

            result = result.max(j.saturating_sub(i + 1));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::max_distance(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
