pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;
use std::mem;

impl Solution {
    fn unwrap_edge(edge: Vec<i32>) -> (usize, usize) {
        let [from, to]: [_; 2] = edge.try_into().ok().unwrap();

        (from as u32 as usize, to as u32 as usize)
    }

    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as u32 as usize;
        let cache_size = n * n;
        let mut buffer = vec![false; cache_size * 2].into_boxed_slice();
        let (mut cache, mut temp) = buffer.split_at_mut(cache_size);

        for edge in prerequisites {
            let (from, to) = Self::unwrap_edge(edge);

            cache[n * from + to] = true;
        }

        for target in cache.iter_mut().step_by(n + 1) {
            *target = true;
        }

        let mut i = 0;

        for middle in 0..n {
            let middle_row = &cache[i..i + n];

            i += n;

            for ((temp_row, source_row), &source_to_middle) in temp
                .chunks_exact_mut(n)
                .zip(cache.chunks_exact(n))
                .zip(cache[middle..].iter().step_by(n))
            {
                for ((target, &source), &middle_to_target) in temp_row.iter_mut().zip(source_row).zip(middle_row) {
                    *target = source || (source_to_middle && middle_to_target);
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        queries
            .into_iter()
            .map(|query| {
                let (from, to) = Self::unwrap_edge(query);

                cache[n * from + to]
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::check_if_prerequisite(num_courses, prerequisites, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
