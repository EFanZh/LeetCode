pub struct Solution;

use std::mem;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut temp_cache = vec![0; triangle.len()];
        let mut rows = triangle.into_iter().rev();
        let mut cache = rows.next().unwrap();

        for row in rows {
            let (&first, rest) = cache.split_first().unwrap();
            let mut prev = first;

            for (target, (&source, val)) in temp_cache.iter_mut().zip(rest.iter().zip(row)) {
                *target = prev.min(source) + val;
                prev = source;
            }

            mem::swap(&mut cache, &mut temp_cache);
        }

        cache[0]
    }
}

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
