pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();
        let mut min = u32::MAX;

        let mins = nums
            .iter()
            .rev()
            .map(|&num| {
                min = min.min(num);

                min
            })
            .collect::<Box<_>>();

        let mut max = 0;

        mins.iter()
            .rev()
            .zip(nums)
            .position(|(&min, num)| {
                max = max.max(num);

                max - min <= k
            })
            .map_or(-1, |i| i as _)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        Self::first_stable_index(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
