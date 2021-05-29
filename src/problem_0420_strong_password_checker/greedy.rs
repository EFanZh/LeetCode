pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_streaks(s: &[u8]) -> (usize, usize, usize) {
        let length = s.len();
        let mut replace_steps = 0;
        let mut zero = 0;
        let mut one = 0;
        let mut iter = s.iter().copied().enumerate();
        let mut start = 0;
        let mut prev = iter.next().unwrap().1;

        for (i, c) in iter {
            if c != prev {
                let streak_length = i - start;

                if streak_length > 2 {
                    replace_steps += streak_length / 3;

                    match streak_length % 3 {
                        0 => zero += 1,
                        1 => one += 1,
                        _ => {}
                    }
                }

                start = i;
                prev = c;
            }
        }

        let streak_length = length - start;

        if streak_length > 2 {
            replace_steps += streak_length / 3;

            match streak_length % 3 {
                0 => zero += 1,
                1 => one += 1,
                _ => {}
            }
        }

        (replace_steps, zero, one)
    }

    pub fn strong_password_checker(password: String) -> i32 {
        let password = password.into_bytes();
        let length = password.len();

        let required_steps = 3
            - usize::from(password.iter().any(u8::is_ascii_lowercase))
            - usize::from(password.iter().any(u8::is_ascii_uppercase))
            - usize::from(password.iter().any(u8::is_ascii_digit));

        if length < 6 {
            required_steps.max(6 - length) as _
        } else {
            let (mut replace_steps, zero, one) = Self::get_streaks(&password);

            if length < 21 {
                required_steps.max(replace_steps) as _
            } else {
                let need_to_delete = length - 20;

                if need_to_delete <= zero {
                    replace_steps -= need_to_delete;
                } else if need_to_delete <= (zero + one) {
                    replace_steps -= zero;
                    replace_steps -= (need_to_delete - zero) / 2;
                } else {
                    replace_steps -= zero;
                    replace_steps -= one;
                    replace_steps = replace_steps.saturating_sub((need_to_delete - (zero + one * 2)) / 3);
                }

                (need_to_delete + replace_steps.max(required_steps)) as _
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn strong_password_checker(password: String) -> i32 {
        Self::strong_password_checker(password)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
