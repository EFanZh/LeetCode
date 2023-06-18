pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn compress(length: u8) -> u8 {
        if length < 10 {
            length.min(2)
        } else if length < 100 {
            3
        } else {
            4
        }
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as u32 as usize;
        let columns = k + 1;
        let mut cache = vec![0; columns * n].into_boxed_slice();

        cache[0] = 1;

        let mut prev_c = s[0];
        let mut chunk_length = 1;

        for (i, &c) in (1..).zip(&s[1..]) {
            // `deletions == 0`.

            if c == prev_c {
                chunk_length += 1;
            } else {
                prev_c = c;
                chunk_length = 1;
            }

            cache[columns * i] = cache
                .get(columns.wrapping_mul(i.wrapping_sub(usize::from(chunk_length))))
                .copied()
                .unwrap_or(0)
                + Self::compress(chunk_length);

            // `deletions > 0`.

            for deletions in 1..columns {
                // Case of deleting `c`.

                let mut min_length = cache[columns * (i - 1) + (deletions - 1)];

                // Cases of keeping `c`.

                let mut chunk_length = 0;
                let mut available_deletions = deletions;

                for (j, &c2) in s[..=i].iter().enumerate().rev() {
                    if c2 == c {
                        chunk_length += 1;
                    } else if available_deletions == 0 {
                        break;
                    } else {
                        available_deletions -= 1;
                    };

                    min_length = min_length.min(
                        cache
                            .get(columns.wrapping_mul(j.wrapping_sub(1)) + available_deletions)
                            .copied()
                            .unwrap_or(0)
                            + Self::compress(chunk_length),
                    );
                }

                cache[columns * i + deletions] = min_length;
            }
        }

        i32::from(*cache.last().unwrap())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        Self::get_length_of_optimal_compression(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
