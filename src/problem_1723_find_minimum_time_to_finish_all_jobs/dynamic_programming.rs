pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let k = k as u8;
        let n = jobs.len();
        let mut costs = vec![0_u32; 1 << n].into_boxed_slice();

        for bits in 1..1 << n {
            costs[bits] = costs[bits & (bits - 1)] + jobs[bits.trailing_zeros() as usize] as u32;
        }

        drop(jobs);

        let mut cache = costs.clone();

        for _ in 1..k {
            for bits in (1..(1 << n)).rev() {
                let mut subset = bits;
                let mut min_cost = u32::MAX;

                loop {
                    subset = bits & (subset - 1);

                    if subset == 0 {
                        break;
                    }

                    min_cost = min_cost.min(cache[bits ^ subset].max(costs[subset]));
                }

                cache[bits] = min_cost;
            }
        }

        *cache.last().unwrap() as _
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
