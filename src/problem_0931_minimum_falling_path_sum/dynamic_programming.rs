pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let (cache, rest) = matrix.split_first_mut().unwrap();
        let mut cache = cache.as_slice();

        for row in rest {
            for (i, (target, &middle)) in row.iter_mut().zip(cache).enumerate() {
                let left = cache.get(i.wrapping_sub(1)).copied().unwrap_or(i32::MAX);
                let right = cache.get(i + 1).copied().unwrap_or(i32::MAX);

                *target += left.min(middle).min(right);
            }

            cache = row.as_slice();
        }

        cache.iter().copied().min().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        Self::min_falling_path_sum(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
