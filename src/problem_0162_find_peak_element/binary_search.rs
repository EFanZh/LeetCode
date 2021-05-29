pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut length = nums.len();

        while length > 1 {
            let half = length / 2;
            let middle = start + half;

            if nums[middle - 1] < nums[middle] {
                start = middle;
                length -= half;
            } else {
                length = half;
            }
        }

        start as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_peak_element(nums: Vec<i32>) -> i32 {
        Self::find_peak_element(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
