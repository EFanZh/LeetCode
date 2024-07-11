pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn unwrap_edge(edge: Vec<i32>) -> (usize, usize) {
        let [from, to]: [_; 2] = edge.try_into().ok().unwrap();

        (from as u32 as usize, to as u32 as usize)
    }

    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as u32 as usize;
        let mut cache = vec![false; n * n].into_boxed_slice();

        for edge in prerequisites {
            let (from, to) = Self::unwrap_edge(edge);

            cache[n * from + to] = true;
        }

        for target in cache.iter_mut().step_by(n + 1) {
            *target = true;
        }

        for middle in 0..n {
            for start in 0..n {
                for end in 0..n {
                    cache[n * start + end] |= cache[n * start + middle] && cache[n * middle + end];
                }
            }
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
