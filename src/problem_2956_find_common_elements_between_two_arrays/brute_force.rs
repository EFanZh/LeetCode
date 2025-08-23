pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(count: &[i32], target: &[i32]) -> i32 {
        let mut buffer = [false; 100];

        for &x in target {
            buffer[x.cast_unsigned() as usize - 1] = true;
        }

        count.iter().filter(|&x| buffer[x.cast_unsigned() as usize - 1]).count() as _
    }

    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        vec![Self::check(&nums1, &nums2), Self::check(&nums2, &nums1)]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        Self::find_intersection_values(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
