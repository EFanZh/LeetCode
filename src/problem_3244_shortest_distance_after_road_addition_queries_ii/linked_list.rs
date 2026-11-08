pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n.cast_unsigned();
        let mut links = (1..n).collect::<Box<_>>();
        let mut count = n - 1;

        queries
            .into_iter()
            .map(|query| {
                let [from, to] = query.try_into().ok().unwrap();
                let from = from.cast_unsigned() as usize;
                let to = to.cast_unsigned();
                let target = &mut links[from];

                if to > *target {
                    let mut to_delete = mem::replace(target, to);

                    loop {
                        to_delete = mem::replace(&mut links[to_delete as usize], u32::MAX);
                        count -= 1;

                        if to_delete == to {
                            break;
                        }
                    }
                }

                count.cast_signed()
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::shortest_distance_after_queries(n, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
