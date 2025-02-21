pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    #[expect(clippy::unnecessary_map_or, reason = "compatibility")]
    fn helper(max_time: u32, jobs: &[u32], usage: &mut [u32]) -> bool {
        jobs.split_first().map_or(true, |(&cost, rest)| {
            for i in 0..usage.len() {
                let used = &mut usage[i];

                if *used + cost <= max_time {
                    *used += cost;

                    if Self::helper(max_time, rest, usage) {
                        return true;
                    }

                    let used = &mut usage[i];

                    *used -= cost;

                    if *used == 0 {
                        break;
                    }
                }
            }

            false
        })
    }

    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let mut jobs = jobs.into_iter().map(|x| x as u32).collect::<Box<_>>();
        let k = k as u32 as usize;

        jobs.sort_unstable_by_key(|&x| Reverse(x));

        let n = jobs.len();
        let mut left = *jobs.last().unwrap();
        let mut right = jobs[..n + 1 - k].iter().sum::<u32>();
        let mut usage = vec![0; k].into_boxed_slice();

        while left < right {
            let middle = (left + right) / 2;

            if Self::helper(middle, &jobs, &mut usage) {
                usage.fill(0);

                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        Self::minimum_time_required(jobs, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
