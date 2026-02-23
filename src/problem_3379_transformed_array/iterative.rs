pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .map(|(i, &offset)| {
                nums[(i.cast_signed() + offset as isize)
                    .rem_euclid(nums.len().cast_signed())
                    .cast_unsigned()]
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        Self::construct_transformed_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
