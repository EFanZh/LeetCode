pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_nums(data: Vec<u8>) -> ([u8; 256], usize) {
        let mut result = [0; 256];
        let mut elements = 0;

        for c in data {
            let slot = &mut result[usize::from(c)];

            if *slot == 0 {
                elements += 1;
            }

            *slot += 1;
        }

        (result, elements)
    }

    pub fn min_window(s: String, t: String) -> String {
        let s = s.into_bytes();
        let (t, t_elements) = Self::count_nums(t.into_bytes());
        let mut window_start = 0;
        let mut window = [0; 256];
        let mut window_elements = 0;
        let mut min_window_start = 0;
        let mut min_window_len = usize::max_value();

        for (i, &c) in s.iter().enumerate() {
            let expected_count = t[usize::from(c)];

            if expected_count != 0 {
                let count = &mut window[usize::from(c)];

                *count += 1;

                if *count == expected_count {
                    window_elements += 1;
                }

                if window_elements == t_elements {
                    let mut iter = s.iter().enumerate().skip(window_start);

                    loop {
                        let (j, &c2) = iter.next().unwrap();
                        let count = &mut window[usize::from(c2)];

                        if *count != 0 {
                            *count -= 1;

                            if *count == t[usize::from(c2)] - 1 {
                                let window_len = i - j + 1;

                                if window_len < min_window_len {
                                    min_window_start = j;
                                    min_window_len = window_len;
                                }

                                window_start = j + 1;
                                window_elements -= 1;

                                break;
                            }
                        }
                    }
                }
            }
        }

        if min_window_len == usize::max_value() {
            String::new()
        } else {
            String::from_utf8(s[min_window_start..min_window_start + min_window_len].to_vec()).unwrap()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
