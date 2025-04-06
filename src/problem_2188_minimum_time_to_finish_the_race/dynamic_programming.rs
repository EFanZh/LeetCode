pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        const QUEUE_BITS: u8 = 5;

        let change_time = change_time as u32;
        let num_laps = num_laps as u32 as usize;
        let mut min_times = [u32::MAX; 16];

        for tire in tires {
            let [f, r] = <[_; 2]>::map(tire.try_into().ok().unwrap(), |x| x as u32);

            let first_min_time = min_times.first_mut().unwrap();

            (*first_min_time) = (*first_min_time).min(f);

            let mut prev_lap_time = f;
            let mut prev_lap_time_sum = f;
            let mut half_lap_time = f;
            let mut half_lap_time_sum = f;
            let mut is_odd_lap = false;

            let max_laps = min_times.len().min(num_laps);

            for target in &mut min_times[1..max_laps] {
                if let Some((lap_time, lap_time_sum)) = prev_lap_time.checked_mul(r).and_then(|lap_time| {
                    prev_lap_time_sum
                        .checked_add(lap_time)
                        .map(|lap_time_sum| (lap_time, lap_time_sum))
                }) {
                    let saved_half_lap_time_sum = half_lap_time_sum;

                    if is_odd_lap {
                        half_lap_time *= r;
                        half_lap_time_sum += half_lap_time;
                    }

                    let other_lap_time_sum = saved_half_lap_time_sum + half_lap_time_sum;

                    if lap_time_sum - other_lap_time_sum < change_time {
                        *target = (*target).min(lap_time_sum);

                        prev_lap_time = lap_time;
                        prev_lap_time_sum += lap_time;
                        is_odd_lap = !is_odd_lap;

                        continue;
                    }
                }

                break;
            }
        }

        let max_laps = min_times.iter().take_while(|&&time| time != u32::MAX).count();
        let mut queue = [0; (1 << QUEUE_BITS)];
        let queue_mask = queue.len() - 1;
        let mut total_laps = 0_usize;

        queue[0] = change_time.wrapping_neg();

        loop {
            total_laps += 1;

            let min_time = (1..).zip(&min_times[..total_laps.min(max_laps)]).fold(
                u32::MAX,
                |min_time, (last_laps, &last_laps_time)| {
                    min_time.min(
                        queue[total_laps.wrapping_sub(last_laps as usize) & queue_mask].wrapping_add(change_time)
                            + last_laps_time,
                    )
                },
            );

            queue[total_laps & queue_mask] = min_time;

            if total_laps == num_laps {
                break;
            }
        }

        queue[total_laps & queue_mask] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        Self::minimum_finish_time(tires, change_time, num_laps)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
