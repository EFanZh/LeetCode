pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;

        satisfaction.sort_unstable();

        let mut sum = 0;
        let mut result = 0;

        for value in satisfaction.into_iter().rev() {
            sum += value;

            if sum <= 0 {
                break;
            }

            result += sum;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        Self::max_satisfaction(satisfaction)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
