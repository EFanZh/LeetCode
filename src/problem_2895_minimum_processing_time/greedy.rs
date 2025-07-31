pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
        let mut processor_time = processor_time;
        let mut tasks = tasks;

        processor_time.sort_unstable();
        tasks.sort_unstable();

        processor_time
            .iter()
            .zip(tasks.iter().rev().step_by(4))
            .fold(0, |max, (processor_time, task)| {
                u32::max(max, (processor_time + task) as _)
            }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
        Self::min_processing_time(processor_time, tasks)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
