pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let k = k.cast_unsigned() as usize;

        nums.iter()
            .enumerate()
            .filter_map(|(i, &num)| {
                [i.wrapping_sub(k), i + k]
                    .iter()
                    .all(|&j| nums.get(j).is_none_or(|&other| num > other))
                    .then_some(num)
            })
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        Self::sum_of_good_numbers(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
