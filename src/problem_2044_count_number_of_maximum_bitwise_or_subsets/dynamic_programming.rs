pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut cache = vec![0; 1 << nums.len()].into_boxed_slice();

        for mask in 1..cache.len() {
            cache[mask] = cache[mask & (mask - 1)] | nums[mask.trailing_zeros() as usize];
        }

        let expected = *cache.last().unwrap();

        cache.iter().filter(|&&x| x == expected).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        Self::count_max_or_subsets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
