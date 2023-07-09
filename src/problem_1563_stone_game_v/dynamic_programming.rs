pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        // Calculate sum.

        let mut sums = stone_value;
        let sums = sums.as_mut_slice();
        let n = sums.len();
        let mut cache = vec![(0_u32, 0_u32, 0_u32); n * n].into_boxed_slice();

        let mut sum = 0;

        for (sum_slot, cache_slot) in sums.iter_mut().zip(&mut *cache) {
            cache_slot.1 = *sum_slot as _;
            cache_slot.2 = *sum_slot as _;

            sum += *sum_slot;
            *sum_slot = sum;
        }

        // Dynamic programming.

        for length in 2..=n {
            let mut split = 0;

            for start in 0..=n - length {
                let end = start + length;
                let start_sum = sums.get(start.wrapping_sub(1)).copied().unwrap_or(0) as u32;
                let end_sum = sums[end - 1] as u32;

                // Find the split point where.

                let mut split_sum_times_2;
                let max_split_sum_times_2 = start_sum + end_sum;

                while {
                    split_sum_times_2 = sums[split] as u32 * 2;

                    split_sum_times_2 < max_split_sum_times_2
                } {
                    split += 1;
                }

                // Calculate the best score.

                let left_candidate = {
                    let left_end = split + usize::from(split_sum_times_2 == max_split_sum_times_2);

                    if left_end == start {
                        0
                    } else {
                        cache[n * (left_end - start - 1) + start].1
                    }
                };

                let right_candidate = {
                    let right_start = split + 1;

                    if right_start == end {
                        0
                    } else {
                        cache[n * (end - right_start - 1) + right_start].2
                    }
                };

                let score = left_candidate.max(right_candidate);
                let new_alt_score = score + (end_sum - start_sum);
                let max_left_alt_score = cache[n * (length - 2) + start].1.max(new_alt_score);
                let max_right_alt_score = cache[n * (length - 2) + start + 1].2.max(new_alt_score);

                cache[n * (length - 1) + start] = (score, max_left_alt_score, max_right_alt_score);
            }
        }

        cache[n * (n - 1)].0 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        Self::stone_game_v(stone_value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
