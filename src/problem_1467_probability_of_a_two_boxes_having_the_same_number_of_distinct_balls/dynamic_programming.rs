pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn combinations(n: usize) -> Box<[u64]> {
        let columns = n / 2 + 1;
        let mut result = vec![0; columns * (n + 1)].into_boxed_slice();
        let mut iter = result.chunks_exact_mut(columns);
        let mut prev_row = iter.next().unwrap();

        prev_row[0] = 1;

        for row in iter {
            let mut top_left = 0;

            for (target, top) in row.iter_mut().zip(prev_row) {
                *target = top_left + *top;
                top_left = *top;
            }

            prev_row = row;
        }

        result
    }

    pub fn get_probability(balls: Vec<i32>) -> f64 {
        const MAX_COLORS: usize = 8;
        const MAX_BALLS_PER_COLOR: usize = 6;
        const MAX_BALLS: usize = MAX_BALLS_PER_COLOR * MAX_COLORS;

        let total_balls = balls.iter().sum::<i32>() as u32 as usize;
        let combinations = Self::combinations(total_balls);
        let combinations_columns = total_balls / 2 + 1;
        let max_left_balls = total_balls / 2;
        let total_colors = balls.len();
        let mut buffer = [0_u64; (MAX_COLORS * 2 + 1) * (MAX_BALLS / 2 + 1) * 2];
        let columns = total_colors * 2 + 1;
        let cache_size = columns * (max_left_balls + 1);
        let (mut cache, mut temp) = buffer[..cache_size * 2].split_at_mut(cache_size);

        cache[total_colors] = 1;

        let mut current_total_balls = 0;

        for current_balls in balls {
            let current_balls = current_balls as u32 as usize;

            current_total_balls += current_balls;

            for (left_count, cache_row) in cache.chunks_exact(columns).take(current_total_balls + 1).enumerate() {
                for (unique_color_diff, &count) in cache_row.iter().enumerate() {
                    if count != 0 {
                        for left_extra in current_total_balls.saturating_sub(max_left_balls + left_count)
                            ..=current_balls.min(max_left_balls.saturating_sub(left_count))
                        {
                            let left_new_color = left_extra != 0;
                            let right_new_color = left_extra != current_balls;

                            let new_unique_color_diff = if left_new_color == right_new_color {
                                unique_color_diff
                            } else if left_new_color {
                                unique_color_diff + 1
                            } else {
                                unique_color_diff - 1
                            };

                            temp[columns * (left_count + left_extra) + new_unique_color_diff] +=
                                count * combinations[combinations_columns * current_balls + left_extra];
                        }
                    }
                }
            }

            mem::swap(&mut cache, &mut temp);
            temp.fill(0);
        }

        let good_count = cache[columns * max_left_balls + total_colors];
        let total_count = combinations[combinations_columns * total_balls + max_left_balls];

        #[expect(clippy::cast_precision_loss, reason = "optimal")]
        let result = good_count as f64 / total_count as f64;

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_probability(balls: Vec<i32>) -> f64 {
        Self::get_probability(balls)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
