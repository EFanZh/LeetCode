pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        _ = n;

        let mut current_time = 0;
        let mut max_work_time = 0;
        let mut result = 0;

        for log in logs {
            let [id, leave_time]: [_; 2] = log.try_into().ok().unwrap();
            let id = id as u32;
            let leave_time = leave_time as u16;
            let work_time = leave_time - current_time;

            match work_time.cmp(&max_work_time) {
                Ordering::Less => {}
                Ordering::Equal => result = result.min(id),
                Ordering::Greater => {
                    max_work_time = work_time;
                    result = id;
                }
            }

            current_time = leave_time;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        Self::hardest_worker(n, logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
