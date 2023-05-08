pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

struct Context<'a> {
    graph: &'a [u16],
    cache: &'a mut [u8],
    k: u8,
}

impl Solution {
    fn combinations(mut bits: u16, mut n: u8, k: u8, base: u16, f: &mut impl FnMut(u16)) {
        if k == 0 {
            f(base);
        } else {
            let next_k = k - 1;

            loop {
                let bit = bits & bits.wrapping_neg();

                bits ^= bit;
                n -= 1;

                Self::combinations(bits, n, next_k, base | bit, f);

                if n == next_k {
                    break;
                }
            }
        }
    }

    fn dfs(context: &mut Context, nodes: u16, nodes_count: u8) -> u8 {
        if nodes == 0 {
            0
        } else {
            let mut candidate = context.cache[usize::from(nodes)];

            if candidate == u8::MAX {
                // Find nodes that has no in degrees.

                let mut start_nodes = nodes;
                let mut iter = nodes;

                loop {
                    let node = iter.trailing_zeros();
                    let node_bit = 1 << node;

                    start_nodes &= context.graph[node as usize];

                    iter ^= node_bit;

                    if iter == 0 {
                        break;
                    }
                }

                // Iterate all combinations.

                let start_nodes_count = start_nodes.count_ones() as u8;

                if start_nodes_count <= context.k {
                    // We have enough parallelism to select all nodes.

                    candidate = Self::dfs(context, nodes & !start_nodes, nodes_count - start_nodes_count);
                } else {
                    // We have to choose `k` nodes out of `start_nodes_count` nodes.

                    let next_nodes_count = nodes_count - context.k;

                    Self::combinations(start_nodes, start_nodes_count, context.k, 0, &mut |start_nodes| {
                        candidate = candidate.min(Self::dfs(context, nodes & !start_nodes, next_nodes_count));
                    });
                }

                candidate += 1;

                context.cache[usize::from(nodes)] = candidate;
            }

            candidate
        }
    }

    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as u8;
        let mut graph = vec![0; usize::from(n)].into_boxed_slice();

        for relation in relations {
            let [from, to]: [_; 2] = relation.try_into().ok().unwrap();

            graph[from as u32 as usize - 1] |= 1 << (to - 1);
        }

        // Pre-invert children so we don’t need to invert them later on.

        for children in &mut *graph {
            *children = !*children;
        }

        let mut cache = vec![u8::MAX; 1 << n].into_boxed_slice();

        // DFS.

        let mut context = Context {
            graph: &graph,
            cache: &mut cache,
            k: k as _,
        };

        i32::from(Self::dfs(&mut context, (1 << n) - 1, n))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::min_number_of_semesters(n, relations, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
