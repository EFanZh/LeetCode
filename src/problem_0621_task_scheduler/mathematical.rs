pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0; 26];

        for &task in &tasks {
            counts[usize::from(task as u8 - b'A')] += 1;
        }

        let max_count = counts.iter().copied().max().unwrap();
        let max_count_tasks = counts.iter().filter(|&&count| count == max_count).count() as i32;

        (tasks.len() as i32).max((max_count - 1) * (n + 1) + max_count_tasks)
    }
}

impl super::Solution for Solution {
    fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        Self::least_interval(tasks, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
