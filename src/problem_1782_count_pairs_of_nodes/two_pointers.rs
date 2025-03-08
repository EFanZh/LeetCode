pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut degrees = vec![0_u32; n as u32 as usize].into_boxed_slice();
        let mut edge_counts = HashMap::<_, u32>::new();

        for edge in edges {
            let [x, y] = edge.try_into().ok().unwrap();
            let x = x as u16 - 1;
            let y = y as u16 - 1;
            let (x, y) = if y < x { (y, x) } else { (x, y) };

            degrees[usize::from(x)] += 1;
            degrees[usize::from(y)] += 1;

            edge_counts.entry((x, y)).and_modify(|count| *count += 1).or_insert(1);
        }

        #[expect(clippy::redundant_clone, reason = "false positive")]
        let mut sorted_degrees = degrees.clone();

        sorted_degrees.sort_unstable();

        let mut result = queries;
        let mut iter = sorted_degrees.iter().copied();
        let left = iter.next().unwrap();

        for target in &mut result {
            let min_incident = *target as u32;
            let mut candidate = 0;
            let mut iter = iter.clone();
            let mut left = left;

            'outer: while let Some(right) = iter.next_back() {
                loop {
                    if left + right > min_incident {
                        candidate += iter.len() as u32 + 1;

                        break;
                    }

                    if let Some(next_left) = iter.next() {
                        left = next_left;
                    } else {
                        break 'outer;
                    }
                }
            }

            for (&(x, y), &count) in &edge_counts {
                let total_degrees = degrees[usize::from(x)] + degrees[usize::from(y)];

                if total_degrees > min_incident && total_degrees <= min_incident + count {
                    candidate -= 1;
                }
            }

            *target = candidate as _;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        Self::count_pairs(n, edges, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
