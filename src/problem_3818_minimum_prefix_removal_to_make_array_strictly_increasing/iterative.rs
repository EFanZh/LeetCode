pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter().copied();

        iter.next_back()
            .and_then(|mut right| {
                iter.rposition(|left| {
                    if left < right {
                        right = left;

                        false
                    } else {
                        true
                    }
                })
            })
            .map_or(0, |i| i as i32 + 1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        Self::minimum_prefix_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
