pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let session_time = session_time as u8;
        let total_combinations = 1 << tasks.len();
        let mut cache = vec![(0_u8, 0_u8); total_combinations].into_boxed_slice();

        cache[0].1 = session_time;

        for combination in 1..total_combinations {
            let mut bits = combination;
            let mut best = (u8::MAX, u8::MAX);

            loop {
                let last_task = bits.trailing_zeros();
                let last_bit = 1 << last_task;
                let prev_session = cache[combination ^ last_bit];
                let task_time = tasks[last_task as usize] as u8;
                let candidate = prev_session.1 + task_time;

                best = best.min(if candidate <= session_time {
                    (prev_session.0, candidate)
                } else {
                    (prev_session.0 + 1, task_time)
                });

                bits ^= last_bit;

                if bits == 0 {
                    break;
                }
            }

            cache[combination] = best;
        }

        i32::from(cache.last().unwrap().0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        Self::min_sessions(tasks, session_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
