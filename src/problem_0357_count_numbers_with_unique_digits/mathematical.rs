pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut result = 0;
        let mut permutations = 1;

        for i in (10 - n..10).rev() {
            result += permutations;
            permutations *= i;
        }

        result * 9 + 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
