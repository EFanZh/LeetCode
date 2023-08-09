pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks
            .into_iter()
            .map(|task| {
                let [actual, minimum]: [_; 2] = task.try_into().ok().unwrap();

                (actual, minimum)
            })
            .collect::<Box<_>>();

        tasks.sort_unstable_by_key(|(actual, minimum)| actual - minimum);

        let mut energy = 0;
        let mut result = 0;

        for &(actual, minimum) in &*tasks {
            if energy < minimum {
                result += minimum - energy;
                energy = minimum;
            }

            energy -= actual;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        Self::minimum_effort(tasks)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
