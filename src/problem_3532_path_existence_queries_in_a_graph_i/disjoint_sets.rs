pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let max_diff = max_diff.cast_unsigned();
        let mut roots = vec![0_u32; n.cast_unsigned() as usize].into_boxed_slice();

        let mut iter = nums
            .iter()
            .copied()
            .map(i32::cast_unsigned)
            .zip(&mut *roots)
            .enumerate();

        let mut prev = (iter.next().unwrap().1.0, 0);

        iter.for_each(|(i, current)| {
            *current.1 = if prev.0 + max_diff < current.0 { i as _ } else { prev.1 };
            prev = (current.0, *current.1);
        });

        queries
            .into_iter()
            .map(|query| {
                let [from, to]: [_; 2] = query.try_into().ok().unwrap();

                roots[from.cast_unsigned() as usize] == roots[to.cast_unsigned() as usize]
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::path_existence_queries(n, nums, max_diff, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
