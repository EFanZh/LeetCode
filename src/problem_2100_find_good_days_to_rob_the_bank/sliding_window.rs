pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let security = security.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let time = time as u32 as usize;
        let mut result = Vec::new();

        if security.len() > time * 2 {
            let (first_chunk, rest) = security.split_at(time);
            let (second_chunk, third_chunk) = rest.split_at(time);
            let mut left_length = 0;
            let mut left_prev = 0;

            for &value in first_chunk.iter().rev() {
                if value < left_prev {
                    break;
                }

                left_length += 1;
                left_prev = value;
            }

            let mut right_length = 0;
            let mut right_prev = u32::MAX;

            for &value in second_chunk.iter().rev() {
                if value > right_prev {
                    break;
                }

                right_length += 1;
                right_prev = value;
            }

            right_prev = security.get((time * 2).wrapping_sub(1)).copied().unwrap_or(u32::MAX);

            for (day, (&left, &right)) in (time as u32..).zip(rest.iter().zip(third_chunk)) {
                if left_prev < left {
                    left_length = 1;
                } else {
                    left_length += 1;
                }

                left_prev = left;

                if right < right_prev {
                    right_length = 1;
                } else {
                    right_length += 1;
                }

                right_prev = right;

                if left_length > time as u32 && right_length > time as u32 {
                    result.push(day as _);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        Self::good_days_to_rob_bank(security, time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
