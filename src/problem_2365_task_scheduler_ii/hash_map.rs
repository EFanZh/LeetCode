pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let space = u64::from(space as u32 + 1);
        let mut prev_days = HashMap::new();
        let mut time = 0;

        for task in tasks {
            time = *prev_days
                .entry(task)
                .and_modify(|prev_day| *prev_day = time.max(*prev_day + space))
                .or_insert(time)
                + 1;
        }

        time as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        Self::task_scheduler_ii(tasks, space)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
