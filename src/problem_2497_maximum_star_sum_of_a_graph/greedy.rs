pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::ops::Add;

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as u32 as usize;
        let mut nodes = vec![Vec::new(); vals.len()];

        for edge in edges {
            let (from, to) = <(_, _)>::from(<[_; 2]>::map(edge.try_into().ok().unwrap(), |x| x as u32 as usize));

            for (left, right) in [(from, to), (to, from)] {
                let node = &mut nodes[left];
                let right_val = vals[right];

                if right_val > 0 {
                    node.push(right_val);
                }
            }
        }

        vals.into_iter()
            .zip(nodes)
            .fold(i32::MIN, |result, (val, mut neighbors)| {
                if neighbors.len() > k {
                    neighbors.select_nth_unstable_by_key(k, |&x| Reverse(x as u32));
                    neighbors.truncate(k);
                }

                result.max(neighbors.into_iter().fold(val, i32::add))
            })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::max_star_sum(vals, edges, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
