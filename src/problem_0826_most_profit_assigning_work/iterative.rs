pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs = difficulty.into_iter().zip(profit).collect::<Vec<_>>();

        jobs.sort_unstable_by_key(|&(d, _)| d);

        let mut worker = worker;

        worker.sort_unstable();

        let mut result = 0;
        let mut jobs_iter = jobs.iter().copied();
        let (mut current_difficulty, mut current_profit) = jobs_iter.next().unwrap();
        let mut max_profit = 0;
        let mut remaining_length = worker.len() as i32;

        for ability in worker {
            while current_difficulty <= ability {
                max_profit = max_profit.max(current_profit);

                if let Some((next_difficulty, next_profit)) = jobs_iter.next() {
                    current_difficulty = next_difficulty;
                    current_profit = next_profit;
                } else {
                    return result + max_profit * remaining_length;
                }
            }

            result += max_profit;
            remaining_length -= 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        Self::max_profit_assignment(difficulty, profit, worker)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
