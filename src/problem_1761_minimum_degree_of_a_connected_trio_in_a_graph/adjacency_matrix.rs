pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as u32 as usize;
        let mut graph = vec![Vec::new(); n].into_boxed_slice();
        let mut matrix = vec![false; n * n].into_boxed_slice();

        for edge in edges {
            let [from, to]: [_; 2] = edge.try_into().ok().unwrap();
            let from = from as u16 - 1;
            let to = to as u16 - 1;

            graph[usize::from(from)].push(to);
            graph[usize::from(to)].push(from);
            matrix[n * usize::from(from) + usize::from(to)] = true;
            matrix[n * usize::from(to) + usize::from(from)] = true;
        }

        let mut result = usize::MAX / 2;

        for (x, x_neighbors) in (0..).zip(&*graph) {
            if x_neighbors.len() < result + 2 {
                for &y in x_neighbors {
                    let y_neighbors = &graph[usize::from(y)];

                    if x_neighbors.len() + y_neighbors.len() < result + 4 {
                        for &z in y_neighbors {
                            if matrix[n * x + usize::from(z)] {
                                result =
                                    result.min(x_neighbors.len() + y_neighbors.len() + graph[usize::from(z)].len() - 6);
                            }
                        }
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        Self::min_trio_degree(n, edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
