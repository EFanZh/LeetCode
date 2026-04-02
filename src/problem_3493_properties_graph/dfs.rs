pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::vec;

impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        let properties = properties
            .into_iter()
            .map(|properties| {
                let mut buckets = [0_usize; usize::div_ceil(101, usize::BITS as _)];

                for property in properties {
                    let property = property.cast_unsigned();
                    let bucket = &mut buckets[(property / usize::BITS) as usize];

                    *bucket |= 1 << (property % usize::BITS);
                }

                buckets
            })
            .collect::<Vec<_>>();

        let k = k.cast_unsigned();
        let mut graph = vec![vec![]; properties.len()].into_boxed_slice();
        let mut iter = properties.iter().enumerate();

        while let Some((lhs, lhs_properties)) = iter.next() {
            iter.clone().for_each(|(rhs, rhs_properties)| {
                if lhs_properties
                    .iter()
                    .zip(rhs_properties)
                    .map(|(lhs_bucket, rhs_bucket)| (lhs_bucket & rhs_bucket).count_ones())
                    .sum::<u32>()
                    >= k
                {
                    graph[lhs].push(rhs);
                    graph[rhs].push(lhs);
                }
            });
        }

        drop(properties);

        let mut graph = graph.into_iter().map(Some).collect::<Box<_>>();

        // DFS.

        let mut stack = Vec::new();
        let mut result = 0;

        for i in 0..graph.len() {
            if let Some(mut neighbors) = graph[i].take() {
                result += 1;

                loop {
                    for neighbor in neighbors {
                        if let Some(neighbors) = graph[neighbor].take() {
                            stack.push(neighbors);
                        }
                    }

                    if let Some(next_neighbors) = stack.pop() {
                        neighbors = next_neighbors;
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::number_of_components(properties, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
