pub mod cumulative_sum;
pub mod cumulative_sum_2;

pub trait NumArray {
    fn new(nums: Vec<i32>) -> Self;
    fn sum_range(&self, i: i32, j: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::NumArray;

    pub fn run<N: NumArray>() {
        let test_cases = [(
            &[-2, 0, 3, -5, 2, -1] as &[_],
            &[((0, 2), 1), ((2, 5), -1), ((0, 5), -3)] as &[_],
        )];

        for (nums, sums) in test_cases {
            let num_array = N::new(nums.to_vec());

            for &((i, j), expected) in sums {
                assert_eq!(num_array.sum_range(i, j), expected);
            }
        }
    }
}
