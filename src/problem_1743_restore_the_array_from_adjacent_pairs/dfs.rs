pub struct Solution;

use std::collections::HashMap;
use std::convert::TryInto;
use std::mem;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let n = adjacent_pairs.len() + 1;
        let mut graph = HashMap::<_, (_, _)>::with_capacity(n);

        for pair in adjacent_pairs {
            let [a, b]: [i32; 2] = pair.as_slice().try_into().unwrap();

            graph
                .entry(a)
                .and_modify(|nexts| nexts.1 = Some(b))
                .or_insert_with(|| (b, None));

            graph
                .entry(b)
                .and_modify(|nexts| nexts.1 = Some(a))
                .or_insert_with(|| (a, None));
        }

        let mut result = Vec::with_capacity(n);

        let (mut prev, mut node) = graph
            .iter()
            .find_map(|(&node, &(a, b))| if b.is_none() { Some((node, a)) } else { None })
            .unwrap();

        result.push(prev);

        loop {
            result.push(node);

            match graph[&node] {
                (_, None) => break,
                (a, Some(b)) => prev = mem::replace(&mut node, if a == prev { b } else { a }),
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        Self::restore_array(adjacent_pairs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
