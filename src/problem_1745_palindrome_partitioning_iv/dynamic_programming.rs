pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.into_bytes().into_boxed_slice();
        let n = s.len();

        if n < 4 {
            true
        } else {
            let mut cache = vec![false; n * (n - 2)].into_boxed_slice();
            let mut cache_row_iter = cache.chunks_exact_mut(n);
            let mut prev_cache_row_1 = cache_row_iter.next().unwrap();
            let mut prev_cache_row_2 = cache_row_iter.next().unwrap();

            // Fill cache of length 1.

            prev_cache_row_1.fill(true);

            // Fill cache of length 2.

            for (target, (x, y)) in prev_cache_row_2.iter_mut().zip(s.iter().zip(&s[1..])) {
                *target = x == y;
            }

            // Fill cache of length > 2.

            for (length, cache_row) in (3..).zip(cache_row_iter) {
                for (target, (middle, (x, y))) in cache_row
                    .iter_mut()
                    .zip(prev_cache_row_1[1..].iter().zip(s.iter().zip(&s[length - 1..])))
                {
                    *target = *middle && x == y;
                }

                prev_cache_row_1 = prev_cache_row_2;
                prev_cache_row_2 = cache_row;
            }

            // Try all solutions.

            for (length_1, &is_valid_1) in (1..n - 1).zip(cache.iter().step_by(n)) {
                if is_valid_1 {
                    let is_valid_2_iter = cache[length_1..].iter().step_by(n);

                    let is_valid_3_iter = cache[1..n * (n - 2 - length_1) + length_1 + 2]
                        .iter()
                        .rev()
                        .step_by(n - 1);

                    for (is_valid_2, is_valid_3) in is_valid_2_iter.zip(is_valid_3_iter) {
                        if *is_valid_2 && *is_valid_3 {
                            return true;
                        }
                    }
                }
            }

            false
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_partitioning(s: String) -> bool {
        Self::check_partitioning(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
