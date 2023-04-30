pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Context<'a> {
    s: &'a [(u8, u8)],
    evil: &'a [u8],
    lookup_table: &'a [usize],
    cache: Box<[u32]>,
    plane_size: usize,
}

impl Solution {
    fn build_lookup_table(s: &str) -> Box<[usize]> {
        let s = s.as_bytes();
        let mut result = vec![0; s.len()].into_boxed_slice();
        let mut i = 1;
        let mut matched = 0;

        while let Some(&c) = s.get(i) {
            loop {
                if s[matched] == c {
                    matched += 1;
                    result[i] = matched;

                    break;
                } else if let Some(&next_matches) = result.get(matched.wrapping_sub(1)) {
                    matched = next_matches;
                } else {
                    break;
                }
            }

            i += 1;
        }

        result
    }

    fn helper(context: &mut Context, boundary: usize, i: usize, matched: usize) -> u32 {
        if matched == context.evil.len() {
            0
        } else if let Some(&(low, high)) = context.s.get(i) {
            let cache_index = context.plane_size * boundary + context.evil.len() * i + matched;
            let mut candidate = context.cache[cache_index];

            if candidate == u32::MAX {
                let low = if boundary & 1 == 0 { b'a' } else { low };
                let high = if boundary & 2 == 0 { b'z' } else { high };

                candidate = 0;

                for c in low..=high {
                    let mut matched = matched;

                    loop {
                        if context.evil[matched] == c {
                            matched += 1;

                            break;
                        } else if let Some(&next_matched) = context.lookup_table.get(matched.wrapping_sub(1)) {
                            matched = next_matched;
                        } else {
                            break;
                        }
                    }

                    candidate += Self::helper(
                        context,
                        boundary & (usize::from(c == low) | (usize::from(c == high) << 1)),
                        i + 1,
                        matched,
                    );

                    candidate = candidate.checked_sub(1_000_000_007).unwrap_or(candidate);
                }

                context.cache[cache_index] = candidate;
            }

            candidate
        } else {
            1
        }
    }

    #[allow(unused_variables)] // Expected.
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let s = s1.into_bytes().into_iter().zip(s2.into_bytes()).collect::<Box<_>>();
        let lookup_table = Self::build_lookup_table(&evil);
        let plane_size = evil.len() * s.len();

        Self::helper(
            &mut Context {
                s: &s,
                evil: evil.as_bytes(),
                lookup_table: &lookup_table,
                cache: vec![u32::MAX; plane_size * 4].into_boxed_slice(),
                plane_size,
            },
            3,
            0,
            0,
        ) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        Self::find_good_strings(n, s1, s2, evil)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
