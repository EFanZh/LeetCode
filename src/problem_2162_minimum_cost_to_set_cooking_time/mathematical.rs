pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check_cost(mut current: u8, move_cost: u32, push_cost: u32, d0: u8, d1: u8, d2: u8, d3: u8) -> u32 {
        let mut result = 0;

        let mut press = |digit| {
            result += push_cost;

            if current != digit {
                current = digit;
                result += move_cost;
            }
        };

        'b4: {
            'b3: {
                'b2: {
                    if d0 == 0 {
                        if d1 == 0 {
                            if d2 == 0 {
                                break 'b4;
                            }

                            break 'b3;
                        }

                        break 'b2;
                    }

                    press(d0);
                }

                press(d1);
            }

            press(d2);
        }

        press(d3);

        result
    }

    pub fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
        let (start_at, move_cost, push_cost, target_seconds) = (
            start_at as u8,
            move_cost as u32,
            push_cost as u32,
            target_seconds as u16,
        );

        let minutes = (target_seconds / 60) as u8;
        let seconds = (target_seconds % 60) as u8;
        let mut m0 = minutes / 10;
        let mut m1 = minutes % 10;
        let mut s0 = seconds / 10;
        let s1 = seconds % 10;

        let mut result = if target_seconds < 6000 {
            Self::check_cost(start_at, move_cost, push_cost, m0, m1, s0, s1)
        } else {
            u32::MAX
        };

        if target_seconds >= 60 && s0 < 4 {
            s0 += 6;

            if m1 == 0 {
                m1 = 9;
                m0 -= 1;
            } else {
                m1 -= 1;
            }

            result = result.min(Self::check_cost(start_at, move_cost, push_cost, m0, m1, s0, s1));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
        Self::min_cost_set_time(start_at, move_cost, push_cost, target_seconds)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
