pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut cache = (false, false, true);
        let mut prev = (0, 0);

        for num in nums {
            cache = (
                cache.1,
                cache.2,
                (cache.1 && prev.1 == num)
                    || (cache.0
                        && ((prev.0 == prev.1 && prev.1 == num) || (prev.0 + 1 == prev.1 && prev.1 + 1 == num))),
            );

            prev = (prev.1, num);
        }

        cache.2
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_partition(nums: Vec<i32>) -> bool {
        Self::valid_partition(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
