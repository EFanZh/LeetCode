pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut min_odd = u32::MAX;
        let mut min_even = u32::MAX;

        for num in nums1 {
            let num = num.cast_unsigned();
            let target = if num & 1 == 0 { &mut min_even } else { &mut min_odd };

            *target = (*target).min(num);
        }

        min_odd == u32::MAX || min_even > min_odd
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn uniform_array(nums1: Vec<i32>) -> bool {
        Self::uniform_array(nums1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
