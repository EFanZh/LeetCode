pub struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        const RESULTS: [i32; 9] = [1, 10, 91, 739, 5275, 32491, 168_571, 712_891, 2_345_851];

        RESULTS[n as usize]
    }
}

impl super::Solution for Solution {
    fn count_numbers_with_unique_digits(n: i32) -> i32 {
        Self::count_numbers_with_unique_digits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
