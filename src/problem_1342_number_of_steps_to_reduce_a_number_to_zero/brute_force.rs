pub struct Solution {}

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut result = 0;

        while num != 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }

            result += 1;
        }

        result
    }
}

impl super::Solution for Solution {
    fn number_of_steps(num: i32) -> i32 {
        Self::number_of_steps(num)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
