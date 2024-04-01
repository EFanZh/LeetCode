pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut counts = HashMap::<_, u32>::new();

        for task in tasks {
            counts.entry(task).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut result = 0;

        for value in counts.into_values() {
            if value == 1 {
                return -1;
            }

            result += (value + 2) / 3;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        Self::minimum_rounds(tasks)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
