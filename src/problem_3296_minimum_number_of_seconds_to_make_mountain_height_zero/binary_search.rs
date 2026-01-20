pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mountain_height = mountain_height.cast_unsigned();
        let worker_times = worker_times.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let min_work_time = worker_times.iter().copied().fold(u32::MAX, u32::min);
        let mut left = 0;
        let mut right = u64::from(min_work_time) * ((1 + u64::from(mountain_height)) * u64::from(mountain_height) / 2);

        while left < right {
            let middle = left.midpoint(right);

            let height = worker_times
                .iter()
                .map(|&work_time| ((((middle * 8) / u64::from(work_time) + 1).isqrt() - 1) / 2) as u32)
                .sum::<u32>();

            if height < mountain_height {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        Self::min_number_of_seconds(mountain_height, worker_times)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
