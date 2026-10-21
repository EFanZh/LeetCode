pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(graph: &[Vec<u32>], node: u32, parent: u32) -> u32 {
        graph[node as usize]
            .iter()
            .copied()
            .filter(|&child| child != parent)
            .map(|child| Self::dfs(graph, child, node))
            .max()
            .map_or(0, |depth| depth + 1)
    }

    fn get_depth(edges: Vec<Vec<i32>>) -> u32 {
        let mut graph = vec![Vec::new(); edges.len() + 1].into_boxed_slice();

        for edge in edges {
            let [x, y] = <[_; 2]>::map(edge.try_into().ok().unwrap(), |x| x.cast_unsigned() - 1);

            graph[x as usize].push(y);
            graph[y as usize].push(x);
        }

        Self::dfs(&graph, 0, u32::MAX)
    }

    fn exp_mod(mut base: u64, mut exponent: u32) -> u64 {
        const MODULUS: u64 = 1_000_000_007;

        let mut result = 1;

        if exponent != 0 {
            loop {
                if exponent & 1 != 0 {
                    result = result * base % MODULUS;

                    if exponent == 1 {
                        break;
                    }
                }

                exponent >>= 1;
                base = base * base % MODULUS;
            }
        }

        result
    }

    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let depth = Self::get_depth(edges);

        Self::exp_mod(2, depth - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        Self::assign_edge_weights(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
