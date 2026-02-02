pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
        let mut source = source.into_bytes();
        let pattern = pattern.as_bytes();
        let pattern_len = pattern.len();
        let target_indices_len = target_indices.len() as u32;

        for i in target_indices {
            source[i.cast_unsigned() as usize] |= 1 << 7;
        }

        let mut cache = vec![u32::MAX / 2; pattern_len].into_boxed_slice();

        for c in source {
            let is_target = c >= 1 << 7;
            let c = c & ((1 << 7) - 1);
            let mut old = 0;
            let iter = cache.iter_mut().zip(pattern);

            if is_target {
                iter.for_each(|(target, &pattern)| {
                    old = mem::replace(target, if pattern == c { (*target).min(old + 1) } else { *target });
                });
            } else {
                iter.for_each(|(target, &pattern)| {
                    old = mem::replace(target, if pattern == c { old } else { *target });
                });
            }
        }

        (target_indices_len - *cache.last().unwrap()).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32 {
        Self::max_removals(source, pattern, target_indices)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
