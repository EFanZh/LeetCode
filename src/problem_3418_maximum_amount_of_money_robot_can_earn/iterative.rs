pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let mut iter = coins.iter();
        let mut prev = (0, 0, 0);

        let mut cache = iter
            .next()
            .into_iter()
            .flatten()
            .map(|&coins| {
                prev = (
                    prev.0 + coins,
                    i32::max(prev.0, prev.1 + coins),
                    i32::max(prev.1, prev.2 + coins),
                );

                prev
            })
            .collect::<Box<_>>();

        for row in iter {
            prev = (i32::MIN / 2, i32::MIN / 2, i32::MIN / 2);

            cache.iter_mut().zip(row).for_each(|(target, &coins)| {
                prev = (
                    i32::max(prev.0, target.0),
                    i32::max(prev.1, target.1),
                    i32::max(prev.2, target.2),
                );

                prev = (
                    prev.0 + coins,
                    i32::max(prev.0, prev.1 + coins),
                    i32::max(prev.1, prev.2 + coins),
                );

                *target = prev;
            });
        }

        i32::max(i32::max(prev.0, prev.1), prev.2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        Self::maximum_amount(coins)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
