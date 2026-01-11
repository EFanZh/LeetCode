pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;
use std::iter;

impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k.cast_unsigned() as usize;

        if k <= queries.len() {
            let mut queries = queries
                .into_iter()
                .map(|query| query.into_iter().map(i32::unsigned_abs).sum::<u32>());

            let mut queue = queries.by_ref().take(k).collect::<BinaryHeap<_>>();

            iter::repeat_n(-1, k - 1)
                .chain(Some(queue.peek().unwrap().cast_signed()))
                .chain(queries.map(|distance| {
                    let mut peek_mut = queue.peek_mut().unwrap();
                    let target = &mut *peek_mut;

                    if distance < *target {
                        *target = distance;
                    }

                    drop(peek_mut);

                    queue.peek().unwrap().cast_signed()
                }))
                .collect()
        } else {
            let n = queries.len();

            drop(queries);

            vec![-1; n]
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        Self::results_array(queries, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
