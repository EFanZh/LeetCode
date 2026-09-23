pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn floyd_warshall(costs: &mut [u32; 26 * 26]) {
        for middle in 0..26 {
            for source in 0..26 {
                for target in 0..26 {
                    let candidate = costs[26 * source + middle].saturating_add(costs[26 * middle + target]);
                    let cost = &mut costs[26 * source + target];

                    *cost = (*cost).min(candidate);
                }
            }
        }
    }

    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut costs = [u32::MAX; 26 * 26];

        original
            .into_iter()
            .zip(changed)
            .zip(cost)
            .for_each(|((old, new), cost)| {
                println!("old = {old}, new = {new}, cost = {cost}");

                let target = &mut costs[usize::from(old as u8 - b'a') * 26 + usize::from(new as u8 - b'a')];

                *target = u32::min(*target, cost.cast_unsigned());
            });

        Self::floyd_warshall(&mut costs);

        source
            .bytes()
            .zip(target.bytes())
            .try_fold(0, |sum, (old, new)| {
                if old == new {
                    Some(sum)
                } else {
                    let cost = costs[usize::from(old - b'a') * 26 + usize::from(new - b'a')];

                    if cost == u32::MAX {
                        None
                    } else {
                        Some(sum + i64::from(cost))
                    }
                }
            })
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        Self::minimum_cost(source, target, original, changed, cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
