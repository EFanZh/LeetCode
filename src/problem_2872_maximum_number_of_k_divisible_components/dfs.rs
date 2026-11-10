pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

struct Context {
    graph: Box<[Vec<u16>]>,
    values: Vec<i32>,
    k: NonZero<u64>,
}

impl Solution {
    fn dfs(context: &Context, node: u16, parent: u16) -> (u64, u32) {
        let node = usize::from(node);

        let (sum, count) = context.graph[node].iter().fold(
            (u64::from(context.values[node].cast_unsigned()), 0),
            |(sum, splits), &child| {
                if child == parent {
                    (sum, splits)
                } else {
                    let (child_sum, child_splits) = Self::dfs(context, child, node as _);

                    (sum + child_sum, splits + child_splits)
                }
            },
        );

        (sum, count + u32::from(sum % context.k == 0))
    }

    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let mut graph = vec![Vec::new(); n.cast_unsigned() as _].into_boxed_slice();

        for edge in edges {
            let [from, to] = <[_; 2]>::map(edge.try_into().ok().unwrap(), i32::cast_unsigned);

            graph[from as usize].push(to as u16);
            graph[to as usize].push(from as u16);
        }

        Self::dfs(
            &Context {
                graph,
                values,
                k: NonZero::new(u64::from(k.cast_unsigned())).unwrap(),
            },
            0,
            u16::MAX,
        )
        .1
        .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        Self::max_k_divisible_components(n, edges, values, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
