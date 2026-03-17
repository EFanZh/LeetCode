pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse(cost: &[Vec<i32>]) -> impl DoubleEndedIterator<Item = [u64; 3]> {
        cost.iter().map(|cost| {
            <[i32; 3]>::try_from(cost.as_slice())
                .unwrap()
                .map(i32::cast_unsigned)
                .map(u64::from)
        })
    }

    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        _ = n;

        let (left, right) = cost.split_at(cost.len() / 2);
        let mut cache = (0, 0, 0, 0, 0, 0);

        Self::parse(left).zip(Self::parse(right).rev()).for_each(
            |([top_1, top_2, top_3], [bottom_1, bottom_2, bottom_3])| {
                cache = (
                    (top_1 + bottom_2 + cache.2.min(cache.3).min(cache.4)),
                    (top_1 + bottom_3 + cache.2.min(cache.4).min(cache.5)),
                    (top_2 + bottom_1 + cache.0.min(cache.1).min(cache.5)),
                    (top_2 + bottom_3 + cache.0.min(cache.4).min(cache.5)),
                    (top_3 + bottom_1 + cache.0.min(cache.1).min(cache.3)),
                    (top_3 + bottom_2 + cache.1.min(cache.2).min(cache.3)),
                );
            },
        );

        cache
            .0
            .min(cache.1)
            .min(cache.2)
            .min(cache.3)
            .min(cache.4)
            .min(cache.5)
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        Self::min_cost(n, cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
