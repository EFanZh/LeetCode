pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(managers: &[i32], inform_times: &[i32], employee: usize, states: &mut [u32]) -> u32 {
        let state = &mut states[employee];

        if *state == u32::MAX {
            let manager = managers[employee] as usize;

            let result = inform_times.get(manager).map_or(0, |&inform_time| {
                Self::helper(managers, inform_times, manager, states) + inform_time as u32
            });

            states[employee] = result;

            result
        } else {
            *state
        }
    }

    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let _ = (n, head_id);
        let n = manager.len();
        let managers = manager.as_slice();
        let inform_times = inform_time.as_slice();
        let mut result = 0;
        let mut states = vec![u32::MAX; n];
        let states = states.as_mut_slice();

        for employee in 0..n {
            result = result.max(Self::helper(managers, inform_times, employee, states));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        Self::num_of_minutes(n, head_id, manager, inform_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
