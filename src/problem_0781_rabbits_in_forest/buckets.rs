pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn next_multiple_of(lhs: u32, rhs: u32) -> u32 {
        match lhs % rhs {
            0 => lhs,
            r => lhs + (rhs - r),
        }
    }

    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();

        for num in answers {
            counter
                .entry(num as u32)
                .and_modify(|count: &mut u32| *count += 1)
                .or_insert(1);
        }

        let mut result = 0;

        for (num, count) in counter {
            result += Self::next_multiple_of(count, num + 1);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_rabbits(answers: Vec<i32>) -> i32 {
        Self::num_rabbits(answers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
