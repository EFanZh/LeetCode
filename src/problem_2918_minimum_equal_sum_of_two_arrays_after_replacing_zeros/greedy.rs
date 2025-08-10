pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn analyze(nums: &[u32]) -> (u64, u64) {
        let mut zeroes = 0;
        let mut sum = 0;

        for &num in nums {
            zeroes += u32::from(num == 0);
            sum += u64::from(num);
        }

        if zeroes == 0 {
            (sum, sum)
        } else {
            (sum + u64::from(zeroes), u64::MAX)
        }
    }

    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let nums1 = nums1.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let nums2 = nums2.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let (min1, max1) = Self::analyze(&nums1);
        let (min2, max2) = Self::analyze(&nums2);

        (if max1 < min2 || max2 < min1 {
            u64::MAX
        } else {
            min1.max(min2)
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        Self::min_sum(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
