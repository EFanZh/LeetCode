pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut temp_cache = Vec::with_capacity(triangle.len());
        let mut cache = Vec::with_capacity(triangle.len());

        for row in triangle {
            temp_cache.resize(row.len(), 0);

            for (i, (target, source)) in temp_cache.iter_mut().zip(row).enumerate() {
                *target = match (cache.get(i.wrapping_sub(1)), cache.get(i)) {
                    (None, None) => source,
                    (None, Some(val)) | (Some(val), None) => source + val,
                    (Some(left), Some(right)) => source + left.min(right),
                };
            }

            mem::swap(&mut cache, &mut temp_cache);
        }

        cache.into_iter().min().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        Self::minimum_total(triangle)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
