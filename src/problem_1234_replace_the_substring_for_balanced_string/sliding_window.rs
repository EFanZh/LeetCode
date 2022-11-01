pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut q_count = 0;
        let mut w_count = 0;
        let mut e_count = 0;
        let mut r_count = 0;

        for &c in s {
            match c {
                b'Q' => q_count += 1,
                b'W' => w_count += 1,
                b'E' => e_count += 1,
                _ => r_count += 1,
            }
        }

        let n = s.len();
        let limit = n / 4;

        let mut invalid = u8::from(q_count > limit)
            + u8::from(w_count > limit)
            + u8::from(e_count > limit)
            + u8::from(r_count > limit);

        if invalid == 0 {
            0
        } else {
            let mut start = 0;
            let mut end = 0;
            let mut result = n;

            for &c in s {
                let count = match c {
                    b'Q' => &mut q_count,
                    b'W' => &mut w_count,
                    b'E' => &mut e_count,
                    _ => &mut r_count,
                };

                *count -= 1;

                if *count == limit {
                    invalid -= 1;
                }

                end += 1;

                while invalid == 0 {
                    result = result.min(end - start);

                    let old = s[start];

                    let count = match old {
                        b'Q' => &mut q_count,
                        b'W' => &mut w_count,
                        b'E' => &mut e_count,
                        _ => &mut r_count,
                    };

                    if *count == limit {
                        invalid += 1;
                    }

                    *count += 1;

                    start += 1;
                }
            }

            result as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn balanced_string(s: String) -> i32 {
        Self::balanced_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
