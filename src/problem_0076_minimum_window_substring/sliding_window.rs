pub struct Solution;

impl Solution {
    fn count_nums(data: Vec<u8>) -> ([u8; 256], usize) {
        let mut result = [0; 256];
        let mut count = 0;

        for c in data {
            let slot = &mut result[usize::from(c)];

            if *slot == 0 {
                count += 1;
            }

            *slot += 1;
        }

        (result, count)
    }

    pub fn min_window(s: String, t: String) -> String {
        let s = s.into_bytes();
        let (t, t_count) = Self::count_nums(t.into_bytes());
        let mut i = 0;

        // Extend the window to the right until it contains all the characters in `t`.

        let mut window = [0; 256];
        let mut satisfied = 0;

        while satisfied != t_count {
            if let Some(c) = s.get(i) {
                let expected_count = t[usize::from(*c)];

                if expected_count != 0 {
                    let slot = &mut window[usize::from(*c)];

                    *slot += 1;

                    if *slot == expected_count {
                        satisfied += 1;
                    }
                }

                i += 1;
            } else {
                return String::new();
            }
        }

        // Now we have `s[..i]` is a valid window, shrink the window as short as possible as long as it’s valid.

        let mut window_start = {
            let mut iter = s.iter().enumerate();

            loop {
                if let Some((j, c2)) = iter.next() {
                    let slot = &mut window[usize::from(*c2)];

                    if *slot != 0 {
                        if *slot == t[usize::from(*c2)] {
                            break j;
                        } else {
                            *slot -= 1;
                        }
                    }
                } else {
                    return String::new();
                }
            }
        };

        // Now we have `s[window_start..i]` being a valid window.

        let mut min_window_start = window_start;
        let mut min_window_len = i - window_start;

        while let Some(c) = s.get(i) {
            let count = &mut window[usize::from(*c)];

            if *count != 0 {
                *count += 1;

                for (j, c2) in s.iter().enumerate().skip(window_start) {
                    let slot = &mut window[usize::from(*c2)];

                    if *slot != 0 {
                        if *slot == t[usize::from(*c2)] {
                            let window_len = i - j + 1;

                            window_start = j;

                            if window_len < min_window_len {
                                min_window_start = window_start;
                                min_window_len = window_len;
                            }

                            break;
                        } else {
                            *slot -= 1;
                        }
                    }
                }
            }

            i += 1;
        }

        String::from_utf8(s[min_window_start..min_window_start + min_window_len].to_vec()).unwrap()
    }
}

impl super::Solution for Solution {
    fn min_window(s: String, t: String) -> String {
        Self::min_window(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
