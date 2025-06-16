pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        let mut nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();

        nums.sort_unstable();

        let mut iter = nums.iter().copied();

        iter.next().map_or(0, |mut prev| {
            iter.fold(u32::MAX, |min, num| min.min(num - mem::replace(&mut prev, num)))
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        Self::find_value_of_partition(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
