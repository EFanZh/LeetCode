pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    #[allow(clippy::option_if_let_else)] // False positive.
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut buckets = Vec::<Vec<_>>::with_capacity(group_sizes.len());
        let mut result = Vec::new();

        for (i, size) in (0..).zip(group_sizes) {
            let size = size as usize;

            let bucket = if let Some(bucket) = buckets.get_mut(size - 1) {
                bucket
            } else {
                buckets.resize_with(size, Vec::new);
                buckets.last_mut().unwrap()
            };

            if bucket.is_empty() {
                bucket.reserve_exact(size);
            }

            bucket.push(i);

            if bucket.len() == size {
                result.push(mem::take(bucket));
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        Self::group_the_people(group_sizes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
