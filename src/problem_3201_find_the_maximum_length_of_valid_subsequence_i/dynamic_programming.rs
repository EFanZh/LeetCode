pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut max_even = 0;
        let mut max_odd = 0;
        let mut max_alt_even = 0;
        let mut max_alt_odd = 0;

        for num in nums {
            (max_even, max_odd, max_alt_even, max_alt_odd) = if num & 1 == 0 {
                (max_even + 1, max_odd, max_alt_even.max(max_alt_odd + 1), max_alt_odd)
            } else {
                (max_even, max_odd + 1, max_alt_even, max_alt_odd.max(max_alt_even + 1))
            }
        }

        max_even.max(max_odd).max(max_alt_even).max(max_alt_odd)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_length(nums: Vec<i32>) -> i32 {
        Self::maximum_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
