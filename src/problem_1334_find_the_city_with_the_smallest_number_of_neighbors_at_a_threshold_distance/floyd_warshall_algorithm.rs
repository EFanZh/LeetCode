pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let distance_threshold = distance_threshold as u32;

        // Floyd–Warshall algorithm.

        let mut cache = vec![u32::MAX; n * n].into_boxed_slice();

        for edge in edges {
            let [from, to, weight]: [_; 3] = edge.try_into().ok().unwrap();
            let from = from as usize;
            let to = to as usize;
            let weight = weight as _;

            cache[n * from + to] = weight;
            cache[n * to + from] = weight;
        }

        for distance in cache.iter_mut().step_by(n + 1) {
            *distance = 0;
        }

        for middle in 0..n {
            for start in 0..n {
                for end in 0..n {
                    cache[n * start + end] =
                        cache[n * start + end].min(cache[n * start + middle].saturating_add(cache[n * middle + end]));
                }
            }
        }

        // Search.

        let mut candidate = n;
        let mut candidate_neighbors = usize::MAX;

        for (i, row) in cache.chunks_exact(n).enumerate().rev() {
            let count = row.iter().filter(|&&distance| distance <= distance_threshold).count();

            if count < candidate_neighbors {
                candidate_neighbors = count;
                candidate = i;
            }
        }

        candidate as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        Self::find_the_city(n, edges, distance_threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
