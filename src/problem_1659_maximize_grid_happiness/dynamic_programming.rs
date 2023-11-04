pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn count_personalities(max_patterns: u8) -> Box<[(u8, u8)]> {
        // See <https://oeis.org/A062756> and <https://oeis.org/A081603>.

        let mut result = vec![(0, 0); usize::from(max_patterns)].into_boxed_slice();

        for i in 1..max_patterns {
            let base = result[usize::from(i / 3)];

            result[usize::from(i)] = (base.0 + (i % 3) % 2, base.1 + (i % 3) / 2);
        }

        result
    }

    fn calculate_valid_patterns(max_patterns: u8, introverts_count: u8, extroverts_count: u8) -> Box<[(u8, u8, u8)]> {
        let personalities = Self::count_personalities(max_patterns);
        let mut result = Vec::with_capacity(personalities.len());

        for (pattern, &counts) in (0..).zip(&*personalities) {
            if counts.0 <= introverts_count && counts.1 <= extroverts_count {
                result.push((pattern, counts.0, counts.1));
            }
        }

        drop(personalities);

        result.into_boxed_slice()
    }

    fn calculate_row_score(mut pattern: u8) -> u8 {
        let mut score = 0;
        let mut prev = 0;

        while pattern != 0 {
            let current = pattern % 3;

            pattern /= 3;

            score += match prev * 3 + current {
                0 | 3 | 6 => 0, // 00 | 10 | 20.
                1 => 12,        // 01.
                2 => 4,         // 02.
                4 => 6,         // 11.
                5 => 3,         // 12.
                7 => 11,        // 21.
                _ => 8,         // 22.
            };

            prev = current;
        }

        score
    }

    fn calculate_vertical_diff(mut top_pattern: u8, mut pattern: u8) -> u8 {
        let mut score = 0_u8;

        while pattern != 0 {
            let top = top_pattern % 3;

            top_pattern /= 3;

            let current = pattern % 3;

            pattern /= 3;

            score = score.wrapping_add(match top * 3 + current {
                0 | 1 | 2 | 3 | 6 => 0,       // 00 | 01 | 02 | 10 | 20.
                4 => 6_u8.wrapping_neg(),     // 11.
                5 | 7 => 1_u8.wrapping_neg(), // 12 | 21.
                _ => 4,                       // 22.
            });
        }

        score
    }

    fn calculate_pattern_combination_scores(valid_patterns: &[(u8, u8, u8)]) -> Box<[u8]> {
        let n = valid_patterns.len();
        let mut scores = vec![0; n * n].into_boxed_slice();

        for (target_row, &(pattern, ..)) in scores.chunks_exact_mut(n).zip(valid_patterns) {
            let base_score = Self::calculate_row_score(pattern);

            for (target, &(top_pattern, ..)) in target_row.iter_mut().zip(valid_patterns) {
                *target = base_score.wrapping_add(Self::calculate_vertical_diff(top_pattern, pattern));
            }
        }

        scores
    }

    pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let (m, n, introverts_count, extroverts_count) =
            (m as u8, n as u8, introverts_count as u8, extroverts_count as u8);

        let (m, n) = if n < m { (n, m) } else { (m, n) };
        let valid_patterns = Self::calculate_valid_patterns(3_u8.pow(u32::from(m)), introverts_count, extroverts_count);
        let patterns = valid_patterns.len();
        let scores = Self::calculate_pattern_combination_scores(&valid_patterns);
        let k2 = usize::from(extroverts_count) + 1;
        let k1 = (usize::from(introverts_count) + 1) * k2;
        let cache_size = patterns * k1;
        let mut buffer = vec![0_u16; cache_size * 2].into_boxed_slice();
        let (mut cache, mut temp) = buffer.split_at_mut(cache_size);

        // First row.

        for (pattern, (&(_, introverts, extroverts), &score)) in
            valid_patterns.iter().zip(scores.iter().step_by(patterns)).enumerate()
        {
            cache[k1 * pattern + k2 * usize::from(introverts) + usize::from(extroverts)] = u16::from(score);
        }

        // Rest rows.

        for _ in 1..n {
            // Bottom pattern.
            for (target_block, (&(_, introverts, extroverts), scores_row)) in temp
                .chunks_exact_mut(k1)
                .zip(valid_patterns.iter().zip(scores.chunks_exact(patterns)))
            {
                let introverts = usize::from(introverts);
                let extroverts = usize::from(extroverts);

                for (total_introverts, target_row) in target_block.chunks_exact_mut(k2).enumerate().skip(introverts) {
                    let top_total_introverts = total_introverts - introverts;

                    for (total_extroverts, target) in target_row.iter_mut().enumerate().skip(extroverts) {
                        let top_total_extroverts = total_extroverts - extroverts;
                        let mut max_score = 0;

                        // Top pattern.

                        for (top_pattern, (&(_, top_introverts, top_extroverts), &score)) in
                            valid_patterns.iter().zip(scores_row).enumerate()
                        {
                            if top_introverts <= top_total_introverts as u8
                                && top_extroverts <= top_total_extroverts as u8
                            {
                                max_score = max_score.max(
                                    cache[k1 * top_pattern + k2 * top_total_introverts + top_total_extroverts]
                                        + u16::from(score),
                                );
                            }
                        }

                        *target = max_score;
                    }
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        i32::from(cache.iter().copied().max().unwrap()) * 10
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        Self::get_max_grid_happiness(m, n, introverts_count, extroverts_count)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
